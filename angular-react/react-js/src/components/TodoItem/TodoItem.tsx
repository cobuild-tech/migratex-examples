import type { TodoType } from '../../types/todo-type';
import deleteIcon from '../../assets/icons/delete-icon.svg';
import './TodoItem.scss';

interface TodoItemProps {
  title: string;
  type: TodoType;
  isDone: boolean;
  id: string;
  onComplete: (id: string) => void;
  onIncomplete: (id: string) => void;
  onDelete: (id: string) => void;
}

export function TodoItem({ title, type, isDone, id, onComplete, onIncomplete, onDelete }: TodoItemProps) {
  function handleClickCheck() {
    if (!isDone) onComplete(id);
    else onIncomplete(id);
  }

  function handleClickDelete() {
    if (confirm(`Delete "${title}"?`)) onDelete(id);
  }

  return (
    <li className={`todo-item ${type} ${isDone ? 'done' : ''}`}>
      <div className="checkbox" onClick={handleClickCheck} />
      <p>{title}</p>
      <button onClick={handleClickDelete}>
        <img alt="Trash icon indicating delete" src={deleteIcon} />
      </button>
    </li>
  );
}
