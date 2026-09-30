import { TestBed } from '@angular/core/testing';

import { LocalStorageService } from './local-storage.service';

describe('LocalStorageService', () => {
  let service: LocalStorageService<{ id: string }>;

  beforeEach(() => {
    localStorage.clear();
    TestBed.configureTestingModule({});
    service = TestBed.inject(LocalStorageService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('returns an empty array when nothing is stored', () => {
    expect(service.getItem('missingKey')).toEqual([]);
  });

  it('stores and retrieves items as JSON', () => {
    const items = [{ id: '1' }, { id: '2' }];
    service.setItem('items', items);
    expect(service.getItem('items')).toEqual(items);
  });

  it('overwrites previously stored items', () => {
    service.setItem('items', [{ id: '1' }]);
    service.setItem('items', [{ id: '2' }]);
    expect(service.getItem('items')).toEqual([{ id: '2' }]);
  });
});
