const API_HOST: string = import.meta.env.VITE_API_HOST ?? 'https://api.realworld.show/api';

let authToken: string | null = null;

/** Called by the session whenever the token changes so every request picks it up immediately. */
export function setAuthToken(token: string | null) {
  authToken = token;
}

/**
 * Validation/API failure. `errors` is a flat list of "attribute message" strings,
 * the same shape the Ember app rendered from `model.errors`.
 */
export class ApiError extends Error {
  constructor(public status: number, public errors: string[]) {
    super(errors.join(', ') || `Request failed with status ${status}`);
  }
}

export function flattenErrors(errors: unknown): string[] {
  if (!errors || typeof errors !== 'object') {
    return [];
  }
  return Object.entries(errors as Record<string, unknown>).flatMap(([attribute, messages]) =>
    (Array.isArray(messages) ? messages : [messages]).map((message) => `${attribute} ${message}`),
  );
}

export async function apiFetch<T>(
  path: string,
  { method = 'GET', body }: { method?: string; body?: unknown } = {},
): Promise<T> {
  const headers: Record<string, string> = {};
  if (authToken) {
    headers.Authorization = `Token ${authToken}`;
  }
  if (body !== undefined) {
    headers['Content-Type'] = 'application/json';
  }

  const response = await fetch(`${API_HOST}${path}`, {
    method,
    headers,
    body: body === undefined ? undefined : JSON.stringify(body),
  });

  const text = await response.text();
  let payload: unknown = {};
  try {
    payload = text ? JSON.parse(text) : {};
  } catch {
    // Non-JSON body (e.g. an HTML error page from a proxy)
    if (response.ok) throw new ApiError(response.status, ['response was not valid JSON']);
  }

  const errors = payload && typeof payload === 'object' ? (payload as { errors?: unknown }).errors : undefined;
  if (!response.ok || errors) {
    throw new ApiError(response.status, flattenErrors(errors));
  }
  return payload as T;
}

/** Messages to show the user for a failed request. */
export function errorMessages(error: unknown): string[] {
  if (error instanceof ApiError && error.errors.length) return error.errors;
  return [error instanceof Error ? error.message : String(error)];
}
