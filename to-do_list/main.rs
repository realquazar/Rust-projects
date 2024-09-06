use std::io;

struct Task {
  id: u32,
  description: String,
}

struct ToDoList {
  tasks: Vec<Task>,
  next_id: u32,
}

impl ToDoList {
  fn new() -> ToDoList {
    tasks: Vec::new(),
    next_id: 1,
  }

  fn add_task(&mut self, description: String) {
    let task = Task {
      id: self.next_id,
      description,    
    };
    self.tasks.push(task);
    self.next_id += 1;
}

  fn update_task(&mut self, id: u32, new_description: String) {
    for task in self.tasks.iter_mut() {
      if task.id == id {
        task.description = new_description;
        return;
      }
  }
  println!("Task not found");
}

  fn delete_task(&mut self, id: u32) {
    self.tasks.retain(|task| task.id != id);
  }

  fn print_tasks(&self) {
    for task in &self.tasks {
      println!("{}: {}", task.id, task.description);
    }
  }
}

fn main() {
  let mut to_do_list = ToDoList::new();

  loop {
    println!("Options:")
    println!("1. Add task");
    println!("2. Update task");
    println!("3. Delete task");
    println!("4. Print tasks");
    println!("5. Quit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match input {
      1 => {
        println!("Enter description:");
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Failed to read description");
        to_do_list.add_task(description.trim().to_string());
      }
      2 => {
        println!("Enter task ID:")
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read id");
        let id: u32 = match id.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };
        println!("Enter new task description:");
        io:stdin().read_line(&mut new_description).expect("Failed to read new description");
      }
      3 => {
        println!("Enter task ID");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read id")
        let id: u32 = match id.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };    
      }
      4 => to_do_list.print_tasks(),
      5 => break;
      _ => println!("Invalid option");      
    }
  }
}

