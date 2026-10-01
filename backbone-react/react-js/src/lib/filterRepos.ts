import type { GithubRepo } from '../api/types';

export const REPO_FILTERS = ['all', 'source', 'fork'] as const;
export type RepoFilter = (typeof REPO_FILTERS)[number];

export function isRepoFilter(value: unknown): value is RepoFilter {
  return REPO_FILTERS.includes(value as RepoFilter);
}

/** The All / Source's / Fork's buttons (`_.where(repos, {fork})` in the Backbone app). */
export function filterRepos(repos: GithubRepo[], filter: RepoFilter) {
  if (filter === 'all') return repos;
  return repos.filter((repo) => repo.fork === (filter === 'fork'));
}
