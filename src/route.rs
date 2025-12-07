use crate::db::data_base::TodosStruct;

pub fn server_health() -> String {
    String::from("Pass : Server up and running")
}

pub fn get_todos(todo_instance: &TodosStruct) -> &Vec<String> {
    todo_instance.get_todos()
}
