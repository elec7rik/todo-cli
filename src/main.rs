extern crate serde;
extern crate serde_json;

use std::{collections::{btree_map::Values, HashMap}, fmt::{format, Error}, fs::OpenOptions, io::Read, vec};
use std::str::FromStr;

fn main() {
    let action = std::env::args().nth(1).expect("Please enter an Action");
    let item = std::env::args().nth(2).expect("Please enter an item");
    println!("{:?} {:?}", action, item);
    let mut todo = Todo::new().expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("{} is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("Error occurred: {}", why),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert (&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("db.json")?;
        // write to the file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn new() -> Result<Todo, std::io::Error> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("db.json")?;
        
        // serialize json as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
