use yew::prelude::*;
use yew_functional::*;

use super::todo_app::{Todo, TodoAction};

#[derive(Properties, Clone, PartialEq)]
pub struct TodoEntryProps {
    pub todo: Todo,
    pub on_change: Callback<TodoAction>,
}

#[function_component(TodoEntry)]
pub fn todo_entry(props: &TodoEntryProps) -> Html {
    let TodoEntryProps {
        todo,
        on_change
    } = props;

    let content_update = {
        let todo = todo.clone();
        let on_change = on_change.clone();
        Callback::from(move |change: InputData| {
            on_change.emit(TodoAction::Edit(Todo {
                task: change.value,
                ..todo.clone()
            }));
        })
    };

    let toggle_done = {
        let todo = todo.clone();
        let on_change = on_change.clone();
        Callback::from(move |_| {
            on_change.emit(TodoAction::Edit(Todo {
                is_done: !todo.is_done,
                ..todo.clone()
            }));
        })
    };

    let delete_todo = {
        let on_change = on_change.clone();
        Callback::from(move |_| {
            on_change.emit(TodoAction::Delete)
        })
    };

    let insert_todo = {
        let on_change = on_change.clone();
        Callback::from(move |_| {
            on_change.emit(TodoAction::Insert);
        })
    };

    html! {
        <div>
            <input oninput=content_update value=todo.task.clone() />
            <input type="checkbox" checked=todo.is_done onclick=toggle_done />
            <button onclick=delete_todo>{ "x" }</button>
            <button onclick=insert_todo>{ "+" }</button>
        </div>
    }
}
