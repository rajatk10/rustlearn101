use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
//use serde_json::{Deserializer, Serializer}

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    loop {
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            let s = s.trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
        eprintln!("Invalid input. Please try again");
    }
}
fn read_u32(prompt: &str) -> u32 {
    loop {
        let s = read_line(prompt);
        match s.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => eprintln!("Invalid number. Please try again"),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

//now implement the TodoList struct

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String) {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };
        self.todos.push(todo);
        self.next_id += 1; //monotonic counter always increments even if todo is removed
    }
    fn list(&self) {
        if self.todos.is_empty() {
            println!("No todos have been added yet");
            return;
        }
        println!("{:<8} | {:<4} | {} ", "STATUS", "ID", "TITLE");
        for todo in &self.todos {
            let status = if todo.completed { "[✓]" } else { " " };
            println!("{:<8}| {:<4} | {} ", status, todo.id, todo.title);
        }
    }
    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        if let Some(todo) = self.todos.iter_mut().find(|x| x.id == id) {
            todo.completed = true;
            Ok(())
        } else {
            Err(format!("Todo with id {} not found", id))
        }
    }
    pub fn remove(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.todos.iter().position(|x| x.id == id) {
            self.todos.remove(pos);
            Ok(())
        } else {
            Err(format!("Todo with id {} not found", id))
        }
    }
    fn save_to(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load_from(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let todo_list:TodoList = serde_json::from_str(&json)?;
        //let next_id = todos.iter().map(|x| x.id).max().unwrap_or(0) + 1;
        Ok(todo_list)
    }
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to("todos.json")
    }
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from("todos.json")
    }
}

pub fn execute() {
    println!("Starting todo CLI App...");
    let mut todo_list = TodoList::load().unwrap_or_else(|_| TodoList::new());
    loop {
        println!("Todo List Manager Started");
        println!("1. Add a todo");
        println!("2. List todos");
        println!("3. Complete todo");
        println!("4. Remove todo");
        println!("5. Save and Exit");
        println!("Choose an option: ");

        let choice = read_u32("Enter your choice: ");
        match choice {
            1 => {
                //println!("Enter todo title: ");
                let title = read_line("Enter todo title: ");
                todo_list.add(title.trim().to_string());
                println!("Todo Added!")
            }
            2 => {
                todo_list.list();
            }
            3 => {
                //println!("Enter todo id to complete: ");
                let id = read_u32("Enter todo id to complete: ");
                match todo_list.complete(id) {
                    Ok(_) => println!("Todo Completed!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            4 => {
                //println!("Enter the todo ID to remove");
                let id = read_u32("Enter todo id to remove: ");
                match todo_list.remove(id) {
                    Ok(_) => println!("Todo Removed!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            5 => {
                println!("Saving the file");
                todo_list.save().expect("Failed to save todo file");
                println!("Saved, Exiting now");
                break;
            }
            _ => {println!("Invalid choice");}
        }
    }
}
//Use this method for unit tests to avoid cluttering main file while still accessing main functions
//Unit tests are close to module and integration tests are separate.
#[cfg(test)]
mod tests;
