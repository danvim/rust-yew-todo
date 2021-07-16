use uuid::Uuid;
use yew::prelude::*;
use yew_functional::*;

use super::{
    todo_app::TodoAction::Insert,
    todo_entry::TodoEntry
};

#[derive(Clone, PartialEq)]
pub struct WithKey<T> {
    pub id: Uuid,
    pub content: T,
}

#[derive(Clone, PartialEq)]
pub struct Todo {
    pub is_done: bool,
    pub task: String,
}

impl Todo {
    pub fn new() -> WithKey<Todo> {
        WithKey {
            id: Uuid::new_v4(),
            content: Todo {
                is_done: false,
                task: String::new(),
            }
        }
    }
}

pub enum TodoAction {
    Insert,
    Edit(Todo),
    Delete,
}

#[function_component(TodoApp)]
pub fn todo_app() -> Html {
    let todos = use_state(|| Vec::<WithKey<Todo>>::new());

    let on_todo_action = {
        let todos = todos.clone();
        move |i: usize, action: TodoAction| -> () {
            match action {
                TodoAction::Insert => {
                    log::info!("Inserted");
                    todos.set([&todos[0..i], &[Todo::new()][..], &todos[i..]].concat());
                }
                TodoAction::Edit(todo) => {
                    log::info!("Edited");
                    todos.set([&todos[0..i], &[WithKey::<Todo> {
                        content: todo,
                        ..todos[i]
                    }][..], &todos[i+1..]].concat());
                }
                TodoAction::Delete => {
                    log::info!("Deleted");
                    todos.set([&todos[0..i], &todos[i+1..]].concat());
                }
            }
        }
    };

    let todo_nodes = todos.iter().enumerate().map(|(i, todo_with_key)| {
        let on_action = {
            let on_todo_action = on_todo_action.clone();
            Callback::once(move |action: TodoAction| on_todo_action(i, action))
        };

        let key = todo_with_key.id.to_string();

        html! {
            <TodoEntry key=key todo=todo_with_key.content.clone() on_change=on_action />
        }
    });

    log::info!("Rendered");
    log::info!("{}", todos.iter().map(|todo| todo.content.task.clone()).collect::<Vec<String>>().join(", "));

    let add_last = {
        let todo_length = todos.len();
        let on_todo_action = on_todo_action.clone();
        Callback::once(move |_| {
            on_todo_action(todo_length, Insert)
        })
    };


    html! {
        <div class="container app">
            <h1>{ "Rust Yew Todo App" }</h1>
            <>
                { for todo_nodes }
            </>
            <button onclick=add_last>{ "Add task" }</button>
        </div>
    }
}
