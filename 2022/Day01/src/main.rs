use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("Could not read to String");
    let mut elf_group = Group::parse_input(file);
    let highest_cal = elf_group.highest_cals();
    println!("Elf {} has the highest amount of calories stored, with {} total calories.",highest_cal.1+1,highest_cal.0.sum());
    let mut top_three = vec![highest_cal.0.sum()];
    elf_group.elves.remove(highest_cal.1);
    let mut count = 0;
    while count < 2 {
        count+=1;
        let new_high = elf_group.highest_cals();
        top_three.push(new_high.0.sum());
        elf_group.elves.remove(new_high.1);
    }
    let mut amount = 0;
    for count in top_three.iter() {amount+=count;}
    println!("Together, the top three elves have {} calories stored.",amount)
}

#[derive(Clone,Debug,PartialEq)]
struct Elf {
    cals: Vec<u64>
}

impl Elf {
    fn new(cals:Vec<u64>) -> Self {
        Self{cals}
    }

    fn sum(&self) -> u64 {
        let mut tot = 0;
        for n in self.cals.iter() {
            tot+=n;
        } tot
    }
}

#[derive(Debug)]
struct Group {
    elves: Vec<Elf>
}

impl Group {
    fn parse_input(string:String) -> Self {
        let mut elves: Vec<Elf> = vec![];
        let mut vec: Vec<u64> = vec![];
        for line in string.lines() {
            match line.is_empty() {
                false => vec.push(line.parse::<u64>().unwrap()),
                true => {
                    let elf = Elf::new(vec.clone());
                    vec.clear();
                    elves.push(elf);
                }
            }
        }
        Group{elves}
    }

    fn highest_cals(&self) -> (&Elf,usize) {
        let mut highest = self.elves.get(0).unwrap();
        for elf in self.elves.iter() {
            if elf.sum() > highest.sum() {highest = elf}
        }
        (highest,self.elves.iter().position(|i| i == highest).unwrap())
    }
}