import { describe, it, expect, beforeEach } from 'vitest';
import { getItem, setItem } from './localStorageService';

describe('localStorageService', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it('returns an empty array when nothing is stored', () => {
    expect(getItem('missingKey')).toEqual([]);
  });

  it('stores and retrieves items as JSON', () => {
    const items = [{ id: '1' }, { id: '2' }];
    setItem('items', items);
    expect(getItem('items')).toEqual(items);
  });

  it('overwrites previously stored items', () => {
    setItem('items', [{ id: '1' }]);
    setItem('items', [{ id: '2' }]);
    expect(getItem('items')).toEqual([{ id: '2' }]);
  });
});
