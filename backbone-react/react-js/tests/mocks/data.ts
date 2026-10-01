import type { GithubEvent, GithubRepo, GithubUser } from '../../src/api/types';

export const KNOWN_USER = 'octocat';

export const user: GithubUser = { username: KNOWN_USER, gravatar_id: '7ad39074b0584bc555d0417ae3e7d974' };

export const events: GithubEvent[] = [
  { id: '1', type: 'PushEvent', repo: { name: 'octocat/hello-world' } },
  { id: '2', type: 'IssuesEvent', payload: { action: 'opened' }, repo: { name: 'octocat/spoon-knife' } },
  { id: '3', type: 'WatchEvent', payload: { action: 'started' }, repo: { name: 'github/linguist' } },
];

export const repos: GithubRepo[] = [
  { id: 1, name: 'hello-world', watchers: 1800, forks: 1700, fork: false },
  { id: 2, name: 'spoon-knife', watchers: 12000, forks: 140000, fork: false },
  { id: 3, name: 'linguist', watchers: 40, forks: 12, fork: true },
];
