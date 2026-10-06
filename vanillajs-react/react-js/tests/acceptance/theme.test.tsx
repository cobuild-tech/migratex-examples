import { visit } from '../helpers/app';

describe('theme toggle', () => {
  test('switches between light and dark and remembers the choice', async () => {
    const app = visit();
    const button = app.$('.js-theme-toggle')!;
    expect(button).toHaveTextContent('🌙Dark Mode🌙');

    await app.user.click(button);
    expect(document.documentElement).toHaveAttribute('data-theme', 'dark');
    expect(button).toHaveTextContent('🌞Light Mode🌞');
    expect(localStorage.getItem('theme')).toBe('dark');

    await app.user.click(button);
    expect(document.documentElement).toHaveAttribute('data-theme', 'light');
    expect(localStorage.getItem('theme')).toBe('light');
  });

  test('starts in the saved theme', () => {
    localStorage.setItem('theme', 'dark');
    const app = visit();
    expect(document.documentElement).toHaveAttribute('data-theme', 'dark');
    expect(app.$('.js-theme-toggle')).toHaveTextContent('Light Mode');
  });
});
