import { createContext, useContext, useMemo, useRef, useState, type ReactNode } from 'react';
import { DEFAULT_AVATAR } from '../api/github';
import type { RepoFilter } from '../lib/filterRepos';

/** A fetch the user asked for: which username, and a click id so repeat clicks refetch. */
export interface Request {
  user: string;
  id: number;
}

interface AppState {
  /** The username input. Lives here so it survives phone navigation, as `#home-page` did. */
  username: string;
  setUsername: (value: string) => void;
  /** The avatar image. Keeps the last found user's picture while the block slides up. */
  avatar: string;
  /** Whether the avatar block (and the Activity/Repositories buttons) is shown. */
  avatarOpen: boolean;
  showAvatar: (src: string) => void;
  hideAvatar: () => void;
  activity: Request | null;
  requestActivity: () => void;
  repos: Request | null;
  requestRepos: () => void;
  repoFilter: RepoFilter | null;
  setRepoFilter: (filter: RepoFilter) => void;
}

const AppStateContext = createContext<AppState | null>(null);

export function AppStateProvider({ children }: { children: ReactNode }) {
  const [username, setUsername] = useState('');
  const [avatar, setAvatar] = useState(DEFAULT_AVATAR);
  const [avatarOpen, setAvatarOpen] = useState(false);
  const [activity, setActivity] = useState<Request | null>(null);
  const [repos, setRepos] = useState<Request | null>(null);
  const [repoFilter, setRepoFilter] = useState<RepoFilter | null>(null);
  const nextId = useRef(0);

  const value = useMemo<AppState>(
    () => ({
      username,
      setUsername,
      avatar,
      avatarOpen,
      showAvatar: (src) => {
        setAvatar(src);
        setAvatarOpen(true);
      },
      hideAvatar: () => setAvatarOpen(false),
      activity,
      requestActivity: () => setActivity({ user: username, id: ++nextId.current }),
      repos,
      // Replaces the categories and clears the repo table (the original's two `page:destroy`s).
      requestRepos: () => {
        setRepos({ user: username, id: ++nextId.current });
        setRepoFilter(null);
      },
      repoFilter,
      setRepoFilter,
    }),
    [username, avatar, avatarOpen, activity, repos, repoFilter],
  );

  return <AppStateContext.Provider value={value}>{children}</AppStateContext.Provider>;
}

export function useAppState() {
  const state = useContext(AppStateContext);
  if (!state) throw new Error('useAppState must be used inside <AppStateProvider>');
  return state;
}
