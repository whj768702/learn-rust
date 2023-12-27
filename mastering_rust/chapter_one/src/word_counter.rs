use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::{collections::HashMap, env};

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }
    fn display(self) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value);
        }
    }
}

pub fn run_word_counter() {
    // 相对路径是mastering_rust目录，不会到src那一级
    let current_dir = env::current_dir().expect("Could not get current directory");
    let filename = "words.txt";
    let mut file_path = current_dir;
    file_path.push(filename);

    println!("File path: {:?}", file_path);
    let file = File::open(file_path).expect("Could not oepn file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }
    word_counter.display();
}
