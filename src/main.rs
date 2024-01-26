use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

struct Dictionary {
    words: HashMap<String, String>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            words: HashMap::new()
        }
    }

    pub fn add_word(&mut self) {
        loop {
            print!("add> ");
            stdout().flush().unwrap();
            let mut word = String::new();
            stdin().read_line(&mut word).unwrap();

            if word.trim() == "exit" {
                break;
            } else {
                let word: Vec<&str> = word.split_whitespace().collect();
                let original = word.get(0).unwrap().trim();
                let translate = word.get(1).unwrap().trim();

                self.words.insert(original.to_string(), translate.to_string());
            }
        }
    }

    pub fn find_word(&mut self) {
        print!("find> ");
        stdout().flush().unwrap();
        let mut word = String::new();
        stdin().read_line(&mut word).unwrap();
        if self.words.contains_key(word.trim()) {
            println!("{}: {}", word.trim(), self.words.get(word.trim()).unwrap());
        } else {
            println!("Word not found!");
        }
    }

    pub fn remove_word(&mut self) {
        print!("remove> ");
        stdout().flush().unwrap();
        let mut word = String::new();
        stdin().read_line(&mut word).unwrap();
        self.words.remove(word.trim());
    }

    pub fn show_words(&mut self) {
        println!("{:?}", self.words);
    }

    pub fn parse_command(&mut self, command: &str) {
        match command {
            "add" => Self::add_word(self),
            "find" => Self::find_word(self),
            "remove" => Self::remove_word(self),
            "list" => Self::show_words(self),
            _ => println!("Command not found!"),
        }
    }
}

fn main() {
    let mut dict = Dictionary::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        if command.trim() == "exit" {
            break;
        } else {
            dict.parse_command(command.trim());
        }
    }
}
