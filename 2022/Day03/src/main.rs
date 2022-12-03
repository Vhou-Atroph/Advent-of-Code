use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("Could not read to String");
    let mut priority_sum = 0;
    let mut pt2_priority_sum = 0;
    let mut l_vec: Vec<Rucksack> = vec![];
    for line in file.lines() {
        let sack = Rucksack::new(line.to_string());
        priority_sum+=get_priority(sack.in_both().as_str());
        if !line.is_empty() {
            l_vec.push(sack.clone());
            if l_vec.len() == 3 {
                let grp = Group::new(l_vec.get(0).unwrap().clone(),
                l_vec.get(1).unwrap().clone(),
                l_vec.get(2).unwrap().clone());
                l_vec.clear();
                pt2_priority_sum+=get_priority(&grp.shared());
            }
        }
    }
    println!("PART 1: {}\nPART 2: {}",priority_sum,pt2_priority_sum)
}

fn get_priority(letter:&str) -> u64 {
    let bytes: u64 = letter.as_bytes()[0].into();
    if bytes > 96 {return bytes - 96} else {return bytes - 38}
}

#[derive(Debug,Clone)]
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
        } unreachable!();
    }
}

struct Group(String,String,String);

impl Group {
    fn new(ruck1:Rucksack,ruck2:Rucksack,ruck3:Rucksack) -> Self {
        Self(format!("{}{}",ruck1.0,ruck1.1),format!("{}{}",ruck2.0,ruck2.1),format!("{}{}",ruck3.0,ruck3.1))
    }

    fn shared(&self) -> String {
        for letter in self.0.chars() {
            if self.1.contains(letter) && self.2.contains(letter) {return letter.to_string()}
        } unreachable!();
    }
}