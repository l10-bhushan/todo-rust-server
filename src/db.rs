pub mod data_base {

    pub struct TodosStruct {
        todo_list: Vec<String>,
    }

    impl TodosStruct {
        pub fn new() -> Self {
            Self {
                todo_list: Vec::new(),
            }
        }

        pub fn get_todos(&self) -> &Vec<String> {
            &self.todo_list
        }

        pub fn add_todo(&mut self, todo_item: String) {
            self.todo_list.push(todo_item);
        }
    }
}
