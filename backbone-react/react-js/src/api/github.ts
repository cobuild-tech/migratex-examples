import type { GithubEvent, GithubRepo, GithubUser } from './types';

export const API_HOST = 'https://api.github.com';

/**
 * The user lookup the Backbone app used (`UserModel.urlRoot`). GitHub still serves it, but
 * `gravatar_id` now comes back empty, so the avatar is Gravatar's default image, as in the original.
 */
export const LEGACY_USER_SEARCH_URL = `${API_HOST}/legacy/user/search/`;

export function gravatarUrl(gravatarId: string) {
  return `https://www.gravatar.com/avatar/${gravatarId}?s=200`;
}

/** The placeholder avatar shown before any user is found. */
export const DEFAULT_AVATAR = gravatarUrl('12d350738eec24a8fdfee3177ee70cda');

async function getJson<T>(url: string): Promise<T> {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`GitHub request failed: ${response.status} ${url}`);
  return response.json() as Promise<T>;
}

/** Returns the first search hit, or an empty object when nothing matched (`UserModel.parse`). */
export async function fetchUser(name: string): Promise<GithubUser> {
  const data = await getJson<{ users: GithubUser[] }>(LEGACY_USER_SEARCH_URL + encodeURIComponent(name));
  return data.users[0] ?? {};
}

export function fetchEvents(user: string) {
  return getJson<GithubEvent[]>(`${API_HOST}/users/${encodeURIComponent(user)}/events`);
}

export function fetchRepos(user: string) {
  return getJson<GithubRepo[]>(`${API_HOST}/users/${encodeURIComponent(user)}/repos`);
}
