import { useState, type FormEvent } from 'react';
import { getItem, setItem } from '../services/localStorageService';
import type { ToDo, TodoType } from '../types/todo-type';

export function useTodos() {
  const [todoList, setTodoList] = useState<ToDo[]>(() => getItem<ToDo>('todoList'));
  const [completeList, setCompleteList] = useState<ToDo[]>(() => getItem<ToDo>('completeList'));
  const [title, setTitle] = useState('');
  const [type, setType] = useState<TodoType>('normal');

  function updateLocalStorage(nextTodoList: ToDo[], nextCompleteList: ToDo[]) {
    setItem('completeList', nextCompleteList);
    setItem('todoList', nextTodoList);
  }

  function handleItemComplete(id: string) {
    const item = todoList.find(item => item.id === id);
    const nextTodoList = todoList.filter(item => item.id !== id);
    let nextCompleteList = completeList;

    if (item) {
      nextCompleteList = [{ ...item, isDone: true }, ...completeList];
    }

    setTodoList(nextTodoList);
    setCompleteList(nextCompleteList);
    updateLocalStorage(nextTodoList, nextCompleteList);
  }

  function handleItemIncomplete(id: string) {
    const item = completeList.find(item => item.id === id);
    const nextCompleteList = completeList.filter(item => item.id !== id);
    let nextTodoList = todoList;

    if (item) {
      nextTodoList = [{ ...item, isDone: false }, ...todoList];
    }

    setCompleteList(nextCompleteList);
    setTodoList(nextTodoList);
    updateLocalStorage(nextTodoList, nextCompleteList);
  }

  function handleItemDelete(id: string) {
    const nextTodoList = todoList.filter(todo => todo.id !== id);
    const nextCompleteList = completeList.filter(todo => todo.id !== id);

    setTodoList(nextTodoList);
    setCompleteList(nextCompleteList);
    updateLocalStorage(nextTodoList, nextCompleteList);
  }

  function resetForm() {
    setType('normal');
    setTitle('');
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!title.trim().length) return;

    const nextTodoList = [
      ...todoList,
      {
        title,
        type,
        isDone: false,
        id: crypto.randomUUID(),
      },
    ];

    setTodoList(nextTodoList);
    setItem('todoList', nextTodoList);
    resetForm();
  }

  return {
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
  };
}
