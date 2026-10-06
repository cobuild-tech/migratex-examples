import { visit } from '../helpers/app';

describe('tip calculator', () => {
  test('starts at $0.00 with Reset disabled', () => {
    const app = visit();
    expect(app.tip()).toBe('0.00');
    expect(app.total()).toBe('0.00');
    expect(app.getByRole('button', { name: 'RESET' })).toBeDisabled();
    expect(app.getByAltText('Splitter')).toBeInTheDocument();
  });

  test('splits a bill with a preset tip', async () => {
    const app = visit();
    await app.bill('142.55');
    await app.preset('15%');
    await app.people('5');
    expect(app.tip()).toBe('4.28');
    expect(app.total()).toBe('32.79');
    expect(app.selected()).toEqual(['15%']);
    expect(app.getByRole('button', { name: '15%' })).toHaveAttribute('aria-pressed', 'true');
  });

  test('uses a custom tip', async () => {
    const app = visit();
    await app.bill('100');
    await app.people('4');
    await app.customTip('20');
    expect(app.tip()).toBe('5.00');
    expect(app.total()).toBe('30.00');
    expect(app.selected()).toEqual(['Custom']);
  });

  test('switches between custom and preset tips', async () => {
    const app = visit();
    await app.bill('100');
    await app.people('1');
    await app.preset('10%');
    await app.customTip('20');
    expect(app.selected()).toEqual(['Custom']);
    expect(app.tip()).toBe('20.00');

    await app.preset('5%');
    expect(app.selected()).toEqual(['5%']);
    expect(app.$('.shareinput')).toHaveValue('');
    expect(app.tip()).toBe('5.00');
  });

  test('shows an error when the number of people is zero', async () => {
    const app = visit();
    await app.bill('100');
    await app.preset('10%');
    await app.people('2');
    expect(app.total()).toBe('55.00');

    await app.people('0');
    expect(app.getByText("Can't be zero")).toBeVisible();
    expect(app.$('.people-input')).toHaveClass('invalid-people');
    expect(app.tip()).toBe('0.00');
    expect(app.total()).toBe('0.00');

    // Changing the bill keeps the error and never shows Infinity.
    await app.bill('120');
    expect(app.getByText("Can't be zero")).toBeVisible();
    expect(app.total()).toBe('0.00');

    await app.people('3');
    expect(app.getByText("Can't be zero")).not.toBeVisible();
    expect(app.$('.people-input')).not.toHaveClass('invalid-people');
    expect(app.total()).toBe('44.00');
  });

  test('never shows NaN, whatever order the fields are filled in', async () => {
    const app = visit();
    await app.people('2');
    expect(app.total()).toBe('0.00');
    await app.bill('30');
    expect(app.total()).toBe('15.00');
    await app.people('');
    expect(app.total()).toBe('0.00');
    await app.bill('');
    await app.people('3');
    expect(app.total()).toBe('0.00');
  });

  test('ignores characters that are not part of a number', async () => {
    const app = visit();
    await app.bill('-1a2.345');
    await app.people('2.5x');
    await app.customTip('1o0');
    expect(app.$('.bill-input')).toHaveValue('12.34');
    expect(app.$('.people-input')).toHaveValue('25');
    expect(app.$('.shareinput')).toHaveValue('10');
  });

  test('labels point at their inputs', () => {
    const app = visit();
    expect(app.getByLabelText('Bill')).toBe(app.$('.bill-input'));
    expect(app.getByLabelText(/Number of People/)).toBe(app.$('.people-input'));
    expect(app.getByLabelText('Select Tip %')).toBe(app.$('.shareinput'));
  });
});
