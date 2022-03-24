mod answer;
mod number;
mod player;

use crate::answer::HBAnswer;
use crate::number::{HBNumber, Three};

fn readln() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string()
}

pub fn arg() {}

pub fn run(_: ()) {
    println!("hit your number.");
    let your_number = read_nums();
    let my_number = rand_nums();

    println!("-- hit & blow game start --");
    loop {
        let think_number = read_nums();
        let hb = hit_and_blow(&my_number, &think_number);
        println!("Y:{} {}", think_number, hb);
        if hb.is_end::<Three>() {
            println!("you win");
            break;
        }

        let think_number = rand_nums();
        let hb = hit_and_blow(&your_number, &think_number);
        println!("I:{} {}", think_number, hb);
        if hb.is_end::<Three>() {
            println!("I win");
            break;
        }
    }
}

fn hit_and_blow(act_number: &Three, think_number: &Three) -> HBAnswer {
    let mut hit = 0;
    let mut blow = 0;
    for (i, a) in act_number.nums().into_iter().enumerate() {
        if let Some(t) = think_number.position(a) {
            if i == t {
                hit += 1;
            } else {
                blow += 1;
            }
        }
    }

    HBAnswer::new(hit, blow)
}

fn rand_gen() -> u8 {
    use rand::random;
    let num: u8 = random();
    num % 10
}

fn rand_nums() -> Three {
    let n1 = rand_gen();
    let n2 = loop {
        let r = rand_gen();
        if n1 != r {
            break r;
        }
    };
    let n3 = loop {
        let r = rand_gen();
        if n1 != r && n2 != r {
            break r;
        }
    };
    Three::new(n1, n2, n3)
}

fn read_nums() -> Three {
    loop {
        let read = readln();
        match TryFrom::<&str>::try_from(&read) {
            Ok(nums) => break nums,
            Err(s) => println!("{}", s),
        }
    }
}
