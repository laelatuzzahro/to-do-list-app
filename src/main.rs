use std::io;
struct ToDoItem {
    id : u64,
    title : String,
    completed : bool,
}

struct ToDoList {
    items : Vec<ToDoItem>,
}

impl ToDoList {

    fn new() -> ToDoList {
        ToDoList { items : Vec::new()}
    }

    fn add_item(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_item = ToDoItem {
            id,
            title: title.clone(),
            completed: false,
        };

        self.items.push(new_item);
        println!("New item added with id {id} and title {title}");
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            print!("Your to do list is empty");
        } else {
            println!("To do list:");
            for item in &self.items {
                let status = if item.completed { "[X]" } else  {"[ ]"};
                println!("{} {} - {}", status, item.id, item.title);
            }
        }
    }

    fn complete_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.completed = true;
            println!("Completed: {}", item.title);
        } else {
            println!("To do not found with id {}", id);
        }

    }

    fn remove_item(&mut self) {
            self.items.pop();
            println!("Successfully deleted")
    }
}

fn main() {
    let mut to_do_list = ToDoList::new();

    loop {
        println!("1. Add To Do");
        println!("2. List To Do");
        println!("3. Complete To Do");
        println!("4. Remove Latest To Do");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter title to do : ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                to_do_list.add_item(title.trim().to_string());
            }

            2 => {
                to_do_list.list_items();
            }

            3 => {
                println!("Enter id to do that completed!");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id : u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                to_do_list.complete_item(id);
            }

            4 => {
                to_do_list.remove_item();
            }

            5 => {
                println!("Exiting Program");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
