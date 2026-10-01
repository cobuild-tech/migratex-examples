/** A user from the legacy search API (only the fields the app reads). */
export interface GithubUser {
  username?: string;
  gravatar_id?: string;
}

export interface GithubEvent {
  id: string;
  type: string;
  payload?: { action?: string };
  repo?: { name?: string };
}

export interface GithubRepo {
  id: number;
  name: string;
  watchers: number;
  forks: number;
  fork: boolean;
}
