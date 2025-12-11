pub mod data_base {
    use uuid::Uuid;
    pub struct TodoDb {
        todos: Vec<TodosStruct>,
    }

    impl TodoDb {
        pub fn new() -> Self {
            Self { todos: vec![] }
        }

        pub fn get_todos(&self) -> &Vec<TodosStruct> {
            &self.todos
        }

        pub fn add_todo(&mut self, todo: TodosStruct) {
            self.todos.push(todo);
        }

        pub fn delete_todo(&mut self, id: String) -> Vec<TodosStruct> {
            let mut new_vec: Vec<TodosStruct> = vec![];
            for item in self.todos.clone() {
                if item.id.to_string() != id {
                    new_vec.push(item);
                }
            }
            new_vec
        }
    }

    #[derive(Clone, Debug)]
    pub struct TodosStruct {
        id: Uuid,
        task: String,
    }

    impl TodosStruct {
        pub fn new(task: String) -> Self {
            Self {
                id: Uuid::new_v4(),
                task,
            }
        }
    }
}
