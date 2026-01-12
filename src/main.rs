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

    fn remove(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}

fn read_input_data() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim().to_string()
}

fn main() {
    let mut db = Database::new();

    loop {
        let input = read_input_data();
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
                let value = db.get(parts[1]);
                match value {
                    Some(v) => println!("{}", v),
                    None => println!("nao encontrado"),
                }
            }
        } else if parts[0].to_uppercase() == "REM" {
            if parts.len() != 2 {
                println!("Comando com estrutura invalida. Tente novamente.");
                continue;
            } else {
                let deleted_value = db.remove(parts[1]);
                match deleted_value {
                    Some(deleted_value) => println!("removido: {}", deleted_value),
                    None => println!("Chave '{}' nao encontrada", parts[1]),
                }
            }
        } else {
            println!("Comando '{}' invalido.", parts[0]);
            continue;
        }
    }
}
