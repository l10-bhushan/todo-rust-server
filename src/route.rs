use crate::{
    BAD_STATUS_LINE,
    db::data_base::{TodoDb, TodosStruct},
    helper::body_parser,
};

pub fn server_health() -> String {
    String::from("Pass : Server up and running")
}

pub fn get_todos(todo_instance: &TodoDb) -> &Vec<TodosStruct> {
    todo_instance.get_todos()
}

pub fn add_todo(new_todo: String, todo_instance: &mut TodoDb) -> &Vec<TodosStruct> {
    let task = body_parser(new_todo);
    let todo = TodosStruct::new(task);
    todo_instance.add_todo(todo);
    todo_instance.get_todos()
}

pub fn delete_todo(body: String, todo_db: &mut TodoDb) -> Vec<TodosStruct> {
    let id = body_parser(body);
    todo_db.delete_todo(id)
}

pub fn return_404_notfound() -> String {
    format!("{BAD_STATUS_LINE}\r\n\r\n")
}
