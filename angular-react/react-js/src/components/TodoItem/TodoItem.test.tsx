import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { TodoItem } from './TodoItem';

describe('TodoItem', () => {
  it('renders the title', () => {
    render(
      <TodoItem
        id="abc"
        title="Buy milk"
        type="normal"
        isDone={false}
        onComplete={vi.fn()}
        onIncomplete={vi.fn()}
        onDelete={vi.fn()}
      />
    );
    expect(screen.getByText('Buy milk')).toBeInTheDocument();
  });

  it('emits onComplete when checking an unfinished item', async () => {
    const onComplete = vi.fn();
    const user = userEvent.setup();
    const { container } = render(
      <TodoItem
        id="abc"
        title="Buy milk"
        type="normal"
        isDone={false}
        onComplete={onComplete}
        onIncomplete={vi.fn()}
        onDelete={vi.fn()}
      />
    );

    await user.click(container.querySelector('.checkbox')!);

    expect(onComplete).toHaveBeenCalledWith('abc');
  });

  it('emits onIncomplete when checking a done item', async () => {
    const onIncomplete = vi.fn();
    const user = userEvent.setup();
    const { container } = render(
      <TodoItem
        id="abc"
        title="Buy milk"
        type="normal"
        isDone={true}
        onComplete={vi.fn()}
        onIncomplete={onIncomplete}
        onDelete={vi.fn()}
      />
    );

    await user.click(container.querySelector('.checkbox')!);

    expect(onIncomplete).toHaveBeenCalledWith('abc');
  });

  it('emits onDelete only after the user confirms', async () => {
    const onDelete = vi.fn();
    const user = userEvent.setup();
    const confirmSpy = vi.spyOn(window, 'confirm');

    render(
      <TodoItem
        id="abc"
        title="Buy milk"
        type="normal"
        isDone={false}
        onComplete={vi.fn()}
        onIncomplete={vi.fn()}
        onDelete={onDelete}
      />
    );

    confirmSpy.mockReturnValueOnce(false);
    await user.click(screen.getByRole('button'));
    expect(onDelete).not.toHaveBeenCalled();

    confirmSpy.mockReturnValueOnce(true);
    await user.click(screen.getByRole('button'));
    expect(onDelete).toHaveBeenCalledWith('abc');
  });
});
