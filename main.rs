use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct Database {
    data: HashMap<u32, User>,
    next_id: u32,
}

impl Database {
    fn new() -> Self {
        Database {
            data: HashMap::new(),
            next_id: 1,
        }
    }

    // Create
    fn create(&mut self, name: String, email: String) {
        let user = User {
            id: self.next_id,
            name,
            email,
        };
        self.data.insert(self.next_id, user.clone());
        println!("User created: {:?}", user);
        self.next_id += 1;
    }

    // Read
    fn read(&self, id: u32) {
        match self.data.get(&id) {
            Some(user) => println!("Found: {:?}", user),
            None => println!("User not found"),
        }
    }

    // Update
    fn update(&mut self, id: u32, name: Option<String>, email: Option<String>) {
        if let Some(user) = self.data.get_mut(&id) {
            if let Some(new_name) = name {
                user.name = new_name;
            }
            if let Some(new_email) = email {
                user.email = new_email;
            }
            println!("User updated: {:?}", user);
        } else {
            println!("User not found");
        }
    }

    // Delete
    fn delete(&mut self, id: u32) {
        if self.data.remove(&id).is_some() {
            println!("User deleted.");
        } else {
            println!("User not found.");
        }
    }

    // List all users
    fn list(&self) {
        for user in self.data.values() {
            println!("{:?}", user);
        }
    }
}

fn main() {
    let mut db = Database::new();

    loop {
        println!("Choose an operation: create, read, update, delete, list, exit");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim();

        match operation {
            "create" => {
                println!("Enter name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                println!("Enter email:");
                let mut email = String::new();
                io::stdin().read_line(&mut email).unwrap();

                db.create(name.trim().to_string(), email.trim().to_string());
            }
            "read" => {
                println!("Enter user ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap();

                db.read(id);
            }
            "update" => {
                println!("Enter user ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap();

                println!("Enter new name (or press Enter to skip):");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                println!("Enter new email (or press Enter to skip):");
                let mut email = String::new();
                io::stdin().read_line(&mut email).unwrap();

                db.update(
                    id,
                    if name.trim().is_empty() {
                        None
                    } else {
                        Some(name.trim().to_string())
                    },
                    if email.trim().is_empty() {
                        None
                    } else {
                        Some(email.trim().to_string())
                    },
                );
            }
            "delete" => {
                println!("Enter user ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: u32 = id.trim().parse().unwrap();

                db.delete(id);
            }
            "list" => {
                db.list();
            }
            "exit" => break,
            _ => println!("Invalid operation"),
        }
    }
}
