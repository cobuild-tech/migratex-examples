import { visit } from '../helpers/app';

describe('reset', () => {
  test('is enabled by any input and clears everything', async () => {
    const app = visit();
    const reset = app.getByRole('button', { name: 'RESET' });

    await app.preset('25%');
    expect(reset).toBeEnabled();

    await app.bill('80');
    await app.people('2');
    await app.customTip('10');
    await app.reset();

    expect(app.$('.bill-input')).toHaveValue('');
    expect(app.$('.people-input')).toHaveValue('');
    expect(app.$('.shareinput')).toHaveValue('');
    expect(app.selected()).toEqual([]);
    expect(app.tip()).toBe('0.00');
    expect(app.total()).toBe('0.00');
    expect(reset).toBeDisabled();
  });

  test('forgets the old tip, so a new bill starts from no tip', async () => {
    const app = visit();
    await app.bill('100');
    await app.preset('50%');
    await app.people('1');
    expect(app.total()).toBe('150.00');

    await app.reset();
    await app.bill('100');
    await app.people('1');
    expect(app.tip()).toBe('0.00');
    expect(app.total()).toBe('100.00');
  });

  test('clicking around the disabled button does nothing', async () => {
    const app = visit();
    await app.preset('10%');
    await app.reset();
    await app.user.click(app.$('.result-btn-div'));
    expect(app.getByRole('button', { name: 'RESET' })).toBeDisabled();

    await app.bill('40');
    await app.user.click(app.$('.result-btn-div'));
    expect(app.$('.bill-input')).toHaveValue('40');
  });
});
