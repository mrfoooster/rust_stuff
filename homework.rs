struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn add_task(&mut self, description: &str) -> usize {
        let id = self.tasks.len(); // Using the length of the vector as ID
        let task = Task::new(id, description.to_string());
        self.tasks.push(task);
        id
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.get_mut(id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        println!("ToDo List:");
        for task in &self.tasks {
            println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
        }
    }
}

fn main() {
    let mut todo_list = ToDoList { tasks: Vec::new() };

    // Adding tasks
    let task1_id = todo_list.add_task("Do laundry");
    let task2_id = todo_list.add_task("Buy groceries");

    // Completing tasks
    todo_list.complete_task(task1_id);
    todo_list.complete_task(task2_id);

    // Listing tasks
    todo_list.list_tasks();
}
