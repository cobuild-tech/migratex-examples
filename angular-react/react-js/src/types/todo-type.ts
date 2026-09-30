export type TodoType = 'normal' | 'important' | 'medium' | 'light';

export interface ToDo {
  title: string;
  isDone: boolean;
  id: string;
  type: TodoType;
}
