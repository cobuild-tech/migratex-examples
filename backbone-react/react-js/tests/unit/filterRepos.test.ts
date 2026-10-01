import { filterRepos, isRepoFilter } from '../../src/lib/filterRepos';
import { repos } from '../mocks/data';

describe('filterRepos', () => {
  it('keeps every repo for "all"', () => {
    expect(filterRepos(repos, 'all')).toEqual(repos);
  });

  it('keeps non-forks for "source"', () => {
    expect(filterRepos(repos, 'source').map((r) => r.name)).toEqual(['hello-world', 'spoon-knife']);
  });

  it('keeps forks for "fork"', () => {
    expect(filterRepos(repos, 'fork').map((r) => r.name)).toEqual(['linguist']);
  });

  it('recognises valid filters only', () => {
    expect(['all', 'source', 'fork'].every(isRepoFilter)).toBe(true);
    expect(isRepoFilter('forks')).toBe(false);
    expect(isRepoFilter(undefined)).toBe(false);
  });
});
