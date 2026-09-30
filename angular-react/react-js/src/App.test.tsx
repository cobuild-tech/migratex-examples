import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import App from './App';

describe('App', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it('renders the app', () => {
    render(<App />);
    expect(screen.getByText('Todo App')).toBeInTheDocument();
  });

  it('ignores submissions with a blank title', async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.type(screen.getByPlaceholderText('Add a task'), '   ');
    await user.click(screen.getByRole('button', { name: '+' }));

    expect(screen.queryByRole('listitem')).not.toBeInTheDocument();
  });

  it('adds a todo and resets the form on submit', async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.type(screen.getByPlaceholderText('Add a task'), 'Buy milk');
    await user.selectOptions(screen.getByRole('combobox'), 'important');
    await user.click(screen.getByRole('button', { name: '+' }));

    expect(screen.getByText('Buy milk')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Add a task')).toHaveValue('');
    expect(screen.getByRole('combobox')).toHaveValue('normal');
  });

  it('moves an item from the todo list to the completed list on complete', async () => {
    const user = userEvent.setup();
    const { container } = render(<App />);

    await user.type(screen.getByPlaceholderText('Add a task'), 'Buy milk');
    await user.click(screen.getByRole('button', { name: '+' }));

    await user.click(container.querySelector('.checkbox')!);

    expect(screen.getByText('Completed')).toBeInTheDocument();
    expect(screen.getByText('Buy milk').closest('li')).toHaveClass('done');
  });

  it('moves an item back from the completed list to the todo list on incomplete', async () => {
    const user = userEvent.setup();
    const { container } = render(<App />);

    await user.type(screen.getByPlaceholderText('Add a task'), 'Buy milk');
    await user.click(screen.getByRole('button', { name: '+' }));
    await user.click(container.querySelector('.checkbox')!);
    await user.click(container.querySelector('.checkbox')!);

    expect(screen.queryByText('Completed')).not.toBeInTheDocument();
    expect(screen.getByText('Buy milk').closest('li')).not.toHaveClass('done');
  });

  it('removes an item from either list on delete', async () => {
    vi.spyOn(window, 'confirm').mockReturnValue(true);
    const user = userEvent.setup();
    render(<App />);

    await user.type(screen.getByPlaceholderText('Add a task'), 'Buy milk');
    await user.click(screen.getByRole('button', { name: '+' }));
    await user.click(screen.getByRole('button', { name: /trash/i }));

    expect(screen.queryByText('Buy milk')).not.toBeInTheDocument();
  });
});
