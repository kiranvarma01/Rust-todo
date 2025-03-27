use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
struct ToDoApp {
    tasks: VecDeque<String>,
}

impl ToDoApp {
    fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push_back(task);
        println!("Task added");
    }

    fn remove_task(&mut self) {
        if self.tasks.is_empty() {
            println!("No Tasks to remove");
            return; // Stop execution
        }

        self.list_tasks();
        println!("Enter the task number to remove");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't read input");

        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= self.tasks.len() {
                let removed_task = self.tasks.remove(index - 1);
                println!("Removed task: {}", removed_task.unwrap());
            } else {
                println!("Invalid task number");
            }
        } else {
            println!("Please enter a valid number");
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to show");
        } else {
            println!("To-Do List:");
            for (i, task) in self.tasks.iter().enumerate() {
                println!("{}. {}", i + 1, task);
            }
        }
    }
}

fn main() {
    let mut todo = ToDoApp::new();

    loop {
        println!("\n📌 To-Do App");
        println!("1️⃣ Add Task");
        println!("2️⃣ Remove Task");
        println!("3️⃣ Show Tasks");
        println!("4️⃣ Exit");
        println!("Enter your choice:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't read input");
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Enter task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Can't read input");
                todo.add_task(task.trim().to_string());
            }
            "2" => todo.remove_task(),
            "3" => todo.list_tasks(),
            "4" => break,
            _ => println!("Invalid choice"),
        }
    }
}
