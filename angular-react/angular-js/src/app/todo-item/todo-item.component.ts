import { Component, EventEmitter, Input, Output } from '@angular/core';

import { TodoType } from '../todo-type';

@Component({
  selector: 'todo-item',
  templateUrl: './todo-item.component.html',
  styleUrls: ['./todo-item.component.scss']
})
export class TodoItemComponent {

  @Input() title: string= '';
  @Input() type: TodoType = 'normal';
  @Input() isDone: boolean = false;
  @Input() id: string = '';

  @Output() onDelete = new EventEmitter<string>();
  @Output() onComplete = new EventEmitter<string>();
  @Output() onIncomplete = new EventEmitter<string>();

  handleClickCheck(){
    if(!this.isDone) this.onComplete.emit(this.id)
    else this.onIncomplete.emit(this.id)
  }

  handleClickDelete(){
    if(confirm(`Delete "${this.title}"?`)) this.onDelete.emit(this.id)
  }
}
