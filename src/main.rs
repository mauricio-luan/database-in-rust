use std::{
    collections::HashMap,
    io::{self, Write},
};

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) -> bool {
        self.map.insert(key, value);
        true
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}

fn main() {
    let mut db = Database::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let input = s.trim();

        if input.is_empty() {
            continue;
        }
        if input == "exit" {
            break;
        }
    }

    // db.set(String::from("nome"), String::from("Mauricio"));

    // let nome = db.get("nome");
    // let sobrenome = db.get("sobrenome");

    // match nome {
    //     Some(n) => println!("Hello, {}!", n),
    //     None => println!("sem nomes"),
    // }

    // match sobrenome {
    //     Some(s) => println!("Hello, {}!", s),
    //     None => println!("sem sobrenomes"),
    // }
}
