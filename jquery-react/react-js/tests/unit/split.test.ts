import { calculateSplit, formatMoney } from '../../src/lib/split';

describe('calculateSplit', () => {
  test('splits the bill and tip per person', () => {
    expect(calculateSplit(142.55, 15, 5)).toEqual({ tipPerPerson: 4.28, totalPerPerson: 32.79 });
    expect(calculateSplit(100, 0, 4)).toEqual({ tipPerPerson: 0, totalPerPerson: 25 });
  });

  test('rounds once, so the total is not a cent off', () => {
    // Rounding the base (3.333 → 3.33) and tip (0.333 → 0.33) separately gave 3.66; the true value is 3.67.
    expect(calculateSplit(10, 10, 3)).toEqual({ tipPerPerson: 0.33, totalPerPerson: 3.67 });
  });

  test('returns null for anything it cannot split', () => {
    expect(calculateSplit(NaN, 15, 2)).toBeNull();
    expect(calculateSplit(-5, 15, 2)).toBeNull();
    expect(calculateSplit(50, 15, 0)).toBeNull();
    expect(calculateSplit(50, 15, 1.5)).toBeNull();
    expect(calculateSplit(50, 15, NaN)).toBeNull();
  });

  test('treats a missing tip as 0%', () => {
    expect(calculateSplit(50, NaN, 2)).toEqual({ tipPerPerson: 0, totalPerPerson: 25 });
  });
});

test('formatMoney always shows two decimals', () => {
  expect(formatMoney(4.3)).toBe('4.30');
  expect(formatMoney(0)).toBe('0.00');
});
