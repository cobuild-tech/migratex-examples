import { useTodos } from './hooks/useTodos';
import { TodoItem } from './components/TodoItem/TodoItem';
import type { TodoType } from './types/todo-type';
import './App.scss';

function App() {
  const {
    todoList,
    completeList,
    title,
    setTitle,
    type,
    setType,
    handleSubmit,
    handleItemComplete,
    handleItemIncomplete,
    handleItemDelete,
  } = useTodos();

  return (
    <div className="container">
      <h1>Todo App</h1>
      <div className="lists">
        <ul>
          {todoList.map(item => (
            <TodoItem
              key={item.id}
              title={item.title}
              isDone={item.isDone}
              id={item.id}
              type={item.type}
              onComplete={handleItemComplete}
              onDelete={handleItemDelete}
              onIncomplete={handleItemIncomplete}
            />
          ))}
        </ul>
        {completeList.length > 0 && (
          <div className="todo">
            <span>Completed</span>
            <ul className="todo">
              {completeList.map(item => (
                <TodoItem
                  key={item.id}
                  title={item.title}
                  isDone={item.isDone}
                  id={item.id}
                  type={item.type}
                  onComplete={handleItemComplete}
                  onDelete={handleItemDelete}
                  onIncomplete={handleItemIncomplete}
                />
              ))}
            </ul>
          </div>
        )}
      </div>
      <form onSubmit={handleSubmit} className="forms">
        <input
          name="title"
          className={type}
          type="text"
          placeholder="Add a task"
          value={title}
          onChange={e => setTitle(e.target.value)}
        />
        <select
          name="type"
          className={type}
          value={type}
          onChange={e => setType(e.target.value as TodoType)}
        >
          <option value="light">Low</option>
          <option value="normal">Normal</option>
          <option value="medium">Medium</option>
          <option value="important">Important</option>
        </select>
        <button>+</button>
      </form>
    </div>
  );
}

export default App;
