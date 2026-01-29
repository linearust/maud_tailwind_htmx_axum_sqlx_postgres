use crate::{
    auth::CurrentUser,
    session::FlashMessage,
    models::todo::{FIELD_TASK, Todo},
    paths,
    views::{components::form, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn todos(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    todos: Vec<Todo>,
    task_value: Option<&str>,
    task_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-2xl mx-auto" {
            h1 class="text-xl mb-3" { "Todos" }

            form method="POST" action=(paths::forms::TODOS) class="mb-3 space-y-3" {
                (form::input("text", FIELD_TASK, "New Todo", task_value, task_error))
                (form::submit_button("Add Todo"))
            }

            @if todos.is_empty() {
                p class="text-gray-500 py-4" { "No todos yet" }
            } @else {
                ul class="space-y-2" {
                    @for todo in todos {
                        (todo_item(&todo))
                    }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Todos", "Manage your todos", content)
}

pub fn todo_item(todo: &Todo) -> Markup {
    html! {
        li class="flex items-center gap-3 py-2 border-b" id={"todo-" (todo.id)} {
            form
                hx-patch={(paths::with_param(paths::actions::TODOS_TODO_ID_TOGGLE, "todo_id", &todo.id))}
                hx-target={"#todo-" (todo.id)}
                hx-swap="outerHTML"
            {
                input
                    type="checkbox"
                    checked[todo.is_done]
                    onchange="this.form.requestSubmit()"
                    class="cursor-pointer";
            }

            span class={
                "flex-1 "
                @if todo.is_done { "line-through text-gray-500" }
            } {
                (todo.task)
            }

            form
                hx-delete={(paths::with_param(paths::actions::TODOS_TODO_ID, "todo_id", &todo.id))}
                hx-confirm="Are you sure?"
                hx-target={"#todo-" (todo.id)}
                hx-swap="outerHTML"
            {
                button
                    type="submit"
                    class="text-red-600 hover:text-red-700"
                {
                    "Delete"
                }
            }
        }
    }
}
