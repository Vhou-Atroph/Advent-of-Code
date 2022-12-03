use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args[1].clone()).expect("Could not read to String");
    let guide = StrategyGuide::new(file);
    let win_count = guide.wincount();
    let win_count2 = guide.wincount_real();
    println!("PART 1:{}\nPART 3:{}",win_count,win_count2)
}

struct StrategyGuide(String);

impl StrategyGuide {
    fn new(string:String) -> Self {
        Self(string)
    }

    fn wincount(&self) -> u64 {
        let mut count = 0;
        for line in self.0.lines() {
            let opp = &line[0..=0];
            let my = &line[2..=2];
            count+=eval(opp,my);
        } count
    }

    fn wincount_real(&self) -> u64 {
        let mut count = 0;
        for line in self.0.lines() {
            let opp = &line[0..=0];
            let my = &line[2..=2];
            count+=eval_part_2(opp,my);
        } count
    }
}

fn eval(x:&str,y:&str) -> u64 {
    match (x,y) {
        ("A","Y") => return 8, // 2 + 6
        ("B","Z") => return 9, // 3 + 6
        ("C","X") => return 7, // 1 + 6
        ("A","X") => return 4, // 1 + 3
        ("B","Y") => return 5, // 2 + 3
        ("C","Z") => return 6, // 3 + 3
        ("A","Z") => return 3, // 3 + 0
        ("B","X") => return 1, // 1 + 0
        ("C","Y") => return 2, // 2 + 0
        _ => unreachable!()
        
    }
}

fn eval_part_2(x:&str,y:&str) -> u64 {
    match (x,y) {
        ("A","Y") => return 4, // 1 + 3 (rock draw)
        ("B","Z") => return 9, // 3 + 6 (scissors win)
        ("C","X") => return 2, // 2 + 0 (paper lose)
        ("A","X") => return 3, // 3 + 0 (scissors lose)
        ("B","Y") => return 5, // 2 + 3 (paper draw)
        ("C","Z") => return 7, // 1 + 6 (rock win)
        ("A","Z") => return 8, // 2 + 6 (paper win)
        ("B","X") => return 1, // 1 + 0 (rock lose)
        ("C","Y") => return 6, // 3 + 3 (scissors draw)
        _ => unreachable!()
        
    }
}