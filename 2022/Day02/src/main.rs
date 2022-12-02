use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("Could not read to String");
    let guide = StrategyGuide::new(file);
    let win_count = guide.wincount();
    let win_count2 = guide.wincount_real();
    println!("PART 1:{}\nPART 3:{}",win_count,win_count2)
}

enum Opponent {
    A, // Rock
    B, // Paper
    C // Scissors
}

enum MyChoice {
    X, // Lose
    Y, // Draw
    Z // Win
}

struct StrategyGuide(String);

impl StrategyGuide {
    fn new(string:String) -> Self {
        Self(string)
    }

    fn wincount(&self) -> u64 {
        let mut count = 0;
        for line in self.0.lines() {
            let opp = parse_opponent(&line[0..=0]);
            let my = parse_my_choice(&line[2..=2]);
            count+=eval(opp,my);
        } count
    }

    fn wincount_real(&self) -> u64 {
        let mut count = 0;
        for line in self.0.lines() {
            let opp = parse_opponent(&line[0..=0]);
            let my = parse_my_choice(&line[2..=2]);
            count+=eval_part_2(opp,my);
        } count
    }
}

fn parse_opponent(str:&str) -> Opponent {
    match str {
        "A" => return Opponent::A,
        "B" => return Opponent::B,
        "C" => return Opponent::C,
        _ => panic!("Unexpected character. Panicking!")
    }
}

fn parse_my_choice(str:&str) -> MyChoice {
    match str {
        "X" => return MyChoice::X,
        "Y" => return MyChoice::Y,
        "Z" => return MyChoice::Z,
        _ => panic!("Unexpected character. Panicking!")
    }
}

fn eval(x:Opponent,y:MyChoice) -> u64 {
    match (x,y) {
        (Opponent::A,MyChoice::Y) => return 8, // 2 + 6
        (Opponent::B,MyChoice::Z) => return 9, // 3 + 6
        (Opponent::C,MyChoice::X) => return 7, // 1 + 6
        (Opponent::A,MyChoice::X) => return 4, // 1 + 3
        (Opponent::B,MyChoice::Y) => return 5, // 2 + 3
        (Opponent::C,MyChoice::Z) => return 6, // 3 + 3
        (Opponent::A,MyChoice::Z) => return 3, // 3 + 0
        (Opponent::B,MyChoice::X) => return 1, // 1 + 0
        (Opponent::C,MyChoice::Y) => return 2, // 2 + 0
        
    }
}

fn eval_part_2(x:Opponent,y:MyChoice) -> u64 {
    match (x,y) {
        (Opponent::A,MyChoice::Y) => return 4, // 1 + 3 (rock draw)
        (Opponent::B,MyChoice::Z) => return 9, // 3 + 6 (scissors win)
        (Opponent::C,MyChoice::X) => return 2, // 2 + 0 (paper lose)
        (Opponent::A,MyChoice::X) => return 3, // 3 + 0 (scissors lose)
        (Opponent::B,MyChoice::Y) => return 5, // 2 + 3 (paper draw)
        (Opponent::C,MyChoice::Z) => return 7, // 1 + 6 (rock win)
        (Opponent::A,MyChoice::Z) => return 8, // 2 + 6 (paper win)
        (Opponent::B,MyChoice::X) => return 1, // 1 + 0 (rock lose)
        (Opponent::C,MyChoice::Y) => return 6, // 3 + 3 (scissors draw)
        
    }
}