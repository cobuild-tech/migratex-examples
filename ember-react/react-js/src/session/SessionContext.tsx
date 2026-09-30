import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { ApiError, setAuthToken } from '../api/client';
import * as api from '../api/endpoints';
import type { User } from '../api/types';

// Same key as the Ember app so existing logins carry over.
export const STORAGE_KEY = 'ember-webapp.token';

interface Session {
  user: User | null;
  isLoggedIn: boolean;
  logIn(email: string, password: string): Promise<User>;
  register(username: string, email: string, password: string): Promise<User>;
  logOut(): void;
  setUser(user: User): void;
}

const SessionContext = createContext<Session | null>(null);

function readStoredToken() {
  try {
    return localStorage.getItem(STORAGE_KEY) || null;
  } catch {
    return null;
  }
}

export function SessionProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(() => {
    const stored = readStoredToken();
    setAuthToken(stored);
    return stored;
  });
  const queryClient = useQueryClient();
  const [user, setUserState] = useState<User | null>(null);
  // Like the Ember application route, block rendering until the stored session is resolved.
  const [ready, setReady] = useState(!token);

  const storeToken = useCallback((value: string | null) => {
    setAuthToken(value);
    setToken(value);
    if (value) {
      localStorage.setItem(STORAGE_KEY, value);
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  }, []);

  useEffect(() => {
    if (ready) return;
    let cancelled = false;
    api
      .getCurrentUser()
      .then((u) => !cancelled && setUserState(u))
      .catch((error) => {
        if (cancelled) return;
        if (error instanceof ApiError && error.status === 401) {
          storeToken(null);
        } else {
          // Transient failure (network, 5xx): keep the stored token for the next load and
          // treat this load as logged out.
          setAuthToken(null);
          setToken(null);
        }
      })
      .finally(() => !cancelled && setReady(true));
    return () => {
      cancelled = true;
    };
  }, [ready, storeToken]);

  const signIn = useCallback(
    (u: User) => {
      storeToken(u.token);
      setUserState(u);
      // Favorited/following flags and feeds were fetched for the previous viewer.
      queryClient.resetQueries();
      return u;
    },
    [storeToken, queryClient],
  );

  const value = useMemo<Session>(
    () => ({
      user,
      isLoggedIn: !!token,
      logIn: (email, password) => api.login(email, password).then(signIn),
      register: (username, email, password) => api.register(username, email, password).then(signIn),
      logOut: () => {
        storeToken(null);
        setUserState(null);
        queryClient.resetQueries();
      },
      setUser: (u) => {
        if (u.token && u.token !== token) storeToken(u.token);
        setUserState(u);
      },
    }),
    [user, token, signIn, storeToken, queryClient],
  );

  if (!ready) return null;
  return <SessionContext.Provider value={value}>{children}</SessionContext.Provider>;
}

export function useSession() {
  const session = useContext(SessionContext);
  if (!session) throw new Error('useSession must be used inside <SessionProvider>');
  return session;
}
