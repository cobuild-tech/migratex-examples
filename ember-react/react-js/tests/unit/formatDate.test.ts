import { formatDate } from '../../src/lib/formatDate';

describe('formatDate', () => {
  it('correctly formats the date', () => {
    expect(formatDate('2019-03-27T17:41:33.076Z')).toBe('March 27, 2019');
  });

  it('handles invalid inputs', () => {
    expect(formatDate(null)).toBe('');
  });
});
