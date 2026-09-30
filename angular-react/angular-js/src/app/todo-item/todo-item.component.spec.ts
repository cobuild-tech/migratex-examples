import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TodoItemComponent } from './todo-item.component';

describe('TodoItemComponent', () => {
  let component: TodoItemComponent;
  let fixture: ComponentFixture<TodoItemComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [TodoItemComponent]
    }).compileComponents();

    fixture = TestBed.createComponent(TodoItemComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('emits onComplete when checking an unfinished item', () => {
    component.id = 'abc';
    component.isDone = false;
    spyOn(component.onComplete, 'emit');

    component.handleClickCheck();

    expect(component.onComplete.emit).toHaveBeenCalledWith('abc');
  });

  it('emits onIncomplete when checking a done item', () => {
    component.id = 'abc';
    component.isDone = true;
    spyOn(component.onIncomplete, 'emit');

    component.handleClickCheck();

    expect(component.onIncomplete.emit).toHaveBeenCalledWith('abc');
  });

  it('emits onDelete only after the user confirms', () => {
    component.id = 'abc';
    spyOn(component.onDelete, 'emit');

    spyOn(window, 'confirm').and.returnValue(false);
    component.handleClickDelete();
    expect(component.onDelete.emit).not.toHaveBeenCalled();

    (window.confirm as jasmine.Spy).and.returnValue(true);
    component.handleClickDelete();
    expect(component.onDelete.emit).toHaveBeenCalledWith('abc');
  });
});
