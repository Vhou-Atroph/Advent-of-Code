use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("Could not read to String");
    let mut priority_sum = 0;
    for line in file.lines() {
        let sack = Rucksack::new(line.to_string());
        priority_sum+=get_priority(sack.in_both().as_str());
    }
    println!("{}",priority_sum)
}

fn get_priority(letter:&str) -> u64 {
    let bytes: u64 = letter.as_bytes()[0].into();
    if bytes > 96 {return bytes - 96} else {return bytes - 38}
}

#[derive(Debug)]
struct Rucksack(String,String);

impl Rucksack {
    fn new(str:String) -> Self {
        let str_half = str.len() / 2;
        let string_half = &str[0..str_half];
        let string_other_half = &str[str_half..str.len()];
        Self(string_half.to_string(),string_other_half.to_string())
    }
    
    fn in_both(&self) -> String {
        for letter in self.0.chars() {
            if self.1.contains(letter) {
                let resp = letter;
                return resp.to_string()
            }
        } return format!("This is unreachable!");
    }
}