use std::cell::{ Cell, RefCell };
use candid::Principal;
use ic_cdk::{ api, init, query, update };
mod types;

thread_local! {
    static TODOS: RefCell<Vec<types::Todo>> = RefCell::new(vec![]);
    static OWNER: Cell<Principal> = Cell::new(Principal::from_slice(&[]));
}

#[init]
fn init() {
    OWNER.with(|owner| owner.set(api::caller()));
}

#[query]
fn get_all_todos(page: usize, page_size: usize) -> String {
    let results_json = TODOS.with(|todos| {
        let list = todos.borrow_mut();
        let start_index = (list.len() - 1).min((page - 1) * page_size);
        let end_index = list.len().min(start_index + page_size);
        let results: Vec<&types::Todo> = list
            .iter()
            .skip(start_index)
            .take(end_index - start_index)
            .collect();
        serde_json::to_string(&results).unwrap()
    });
    results_json
}

#[query]
fn get_todo_by_id(id: String) -> Option<String> {
    TODOS.with(|todos| {
        let list = todos.borrow_mut();
        let mut response = None;
        for todo in list.iter() {
            if todo.id == id {
                response = Some(serde_json::to_string(&todo).unwrap());
            }
        }
        response
    })
}

#[update]
fn create_todo(todo_str: String) -> Option<String> {
    let task_result: Result<types::CreateTodo, serde_json::Error> = serde_json::from_str(&todo_str);
    task_result
        .map(|task| {
            let id = TODOS.with(|t| t.borrow_mut().len().to_string());
            let new_task = types::Todo { text: task.text, id };
            TODOS.with(|todos| {
                let mut list = todos.borrow_mut();
                list.push(new_task.clone());
            });
            serde_json::to_string(&new_task).unwrap()
        })
        .ok()
}

#[update]
fn update_todo(todo_str: String) -> bool {
    let task_result = serde_json::from_str(&todo_str);
    task_result
        .map(|task: types::Todo| {
            TODOS.with(|todos| {
                let mut list = todos.borrow_mut();
                for t in list.iter_mut() {
                    if t.id == task.id {
                        *t = task;
                        break;
                    }
                }
            })
        })
        .is_ok()
}

#[update]
fn delete_todo(id: String) -> bool {
    TODOS.with(|todos| {
        let mut list = todos.borrow_mut();
        list.retain(|t| t.id != id);
    });
    true
}
