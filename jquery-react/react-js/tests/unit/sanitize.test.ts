import { money, percent, wholeNumber } from '../../src/lib/sanitize';

test('money accepts up to two decimals', () => {
  for (const ok of ['', '0', '142', '142.', '142.5', '142.55']) expect(money(ok)).toBe(ok);
  for (const bad of ['142.555', '1.2.3', '-5', 'abc', '1e3', ' 1']) expect(money(bad)).toBeNull();
});

test('percent accepts a decimal', () => {
  for (const ok of ['', '12', '12.5', '12.125']) expect(percent(ok)).toBe(ok);
  for (const bad of ['1.2.3', '-5', '%', 'x']) expect(percent(bad)).toBeNull();
});

test('wholeNumber accepts digits only', () => {
  for (const ok of ['', '0', '12']) expect(wholeNumber(ok)).toBe(ok);
  for (const bad of ['1.5', '-1', 'two']) expect(wholeNumber(bad)).toBeNull();
});
