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

        let parts = input.split_whitespace().collect::<Vec<&str>>();

        if parts[0].to_uppercase() == "SET" {
            if parts.len() != 3 {
                println!("Comando com estrutura invalida. Tente novamente.");
                continue;
            } else {
                db.set(String::from(parts[1]), String::from(parts[2]));
            }
        } else if parts[0].to_uppercase() == "GET" {
            if parts.len() != 2 {
                println!("Comando com estrutura invalida. Tente novamente.");
                continue;
            } else {
                let rs = db.get(parts[1]);
                match rs {
                    Some(rs) => println!("{}", rs),
                    None => println!("nao encontrado"),
                }
            }
        }
    }
}
