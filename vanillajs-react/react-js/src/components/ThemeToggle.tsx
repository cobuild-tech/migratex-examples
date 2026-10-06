import type { Theme } from '../hooks/useTheme';

export function ThemeToggle({ theme, onToggle }: { theme: Theme; onToggle: () => void }) {
  const icon = theme === 'light' ? '🌙' : '🌞';
  return (
    <button className="theme-toggle js-theme-toggle" id="theme-toggle" onClick={onToggle}>
      <span className="js-theme-toggle__icon">{icon}</span>
      <span className="js-theme-toggle__text">{theme === 'light' ? 'Dark Mode' : 'Light Mode'}</span>
      <span className="js-theme-toggle__icon_2">{icon}</span>
    </button>
  );
}
