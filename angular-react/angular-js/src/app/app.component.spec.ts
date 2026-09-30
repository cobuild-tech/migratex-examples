import { ComponentFixture, TestBed } from '@angular/core/testing';
import { FormsModule } from '@angular/forms';

import { AppComponent } from './app.component';
import { TodoItemComponent } from './todo-item/todo-item.component';

describe('AppComponent', () => {
  let component: AppComponent;
  let fixture: ComponentFixture<AppComponent>;

  beforeEach(async () => {
    localStorage.clear();
    await TestBed.configureTestingModule({
      imports: [FormsModule],
      declarations: [AppComponent, TodoItemComponent]
    }).compileComponents();

    fixture = TestBed.createComponent(AppComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create the app', () => {
    expect(component).toBeTruthy();
  });

  it('ignores submissions with a blank title', () => {
    component.title = '   ';
    component.handleSubmit({ preventDefault: () => {} } as SubmitEvent);
    expect(component.todoList.length).toBe(0);
  });

  it('adds a todo and resets the form on submit', () => {
    component.title = 'Buy milk';
    component.type = 'important';

    component.handleSubmit({ preventDefault: () => {} } as SubmitEvent);

    expect(component.todoList.length).toBe(1);
    expect(component.todoList[0].title).toBe('Buy milk');
    expect(component.todoList[0].type).toBe('important');
    expect(component.title).toBe('');
    expect(component.type).toBe('normal');
  });

  it('moves an item from todoList to completeList on complete', () => {
    component.title = 'Buy milk';
    component.handleSubmit({ preventDefault: () => {} } as SubmitEvent);
    const id = component.todoList[0].id;

    component.handleItemComplete(id);

    expect(component.todoList.length).toBe(0);
    expect(component.completeList.length).toBe(1);
    expect(component.completeList[0].isDone).toBeTrue();
  });

  it('moves an item back from completeList to todoList on incomplete', () => {
    component.title = 'Buy milk';
    component.handleSubmit({ preventDefault: () => {} } as SubmitEvent);
    const id = component.todoList[0].id;
    component.handleItemComplete(id);

    component.handleItemIncomplete(id);

    expect(component.completeList.length).toBe(0);
    expect(component.todoList.length).toBe(1);
    expect(component.todoList[0].isDone).toBeFalse();
  });

  it('removes an item from either list on delete', () => {
    component.title = 'Buy milk';
    component.handleSubmit({ preventDefault: () => {} } as SubmitEvent);
    const id = component.todoList[0].id;
    component.handleItemComplete(id);

    component.handleItemDelete(id);

    expect(component.todoList.length).toBe(0);
    expect(component.completeList.length).toBe(0);
  });
});
