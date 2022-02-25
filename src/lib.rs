fn readln() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string()
}

pub fn arg() {}

pub fn run(_: ()) {
    println!("hit your number");
    let your_number = read_nums();
    let my_number = rand_nums();

    println!("-- hit & blow --");
    loop {
        println!("think my number");
        let think_number = read_nums();
        let hb = hit_and_blow(my_number, think_number);
        println!("you think: {}", hb);
        if hb.is_end() {
            println!("you win");
            break;
        }

        println!("I think your number");
        let think_number = rand_nums();
        let hb = hit_and_blow(your_number, think_number);
        println!("I think: {}", hb);
        if hb.is_end() {
            println!("I win");
            break;
        }
    }
}

struct HitBlow {
    thinked: [u8; 3],
    hit: u8,
    blow: u8,
}

impl HitBlow {
    fn is_end(&self) -> bool {
        self.hit == 3
    }
}

use std::fmt::{Display, Result};
impl Display for HitBlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Think: {}{}{}, Hit: {}, Blow: {}",
            self.thinked[0], self.thinked[1], self.thinked[2], self.hit, self.blow
        )
    }
}

fn hit_and_blow(act_number: [u8; 3], think_number: [u8; 3]) -> HitBlow {
    let mut hit = 0;
    let mut blow = 0;
    for i in 0..3 {
        if think_number.contains(&act_number[i]) {
            if think_number[i] == act_number[i] {
                hit += 1;
            } else {
                blow += 1;
            }
        }
    }

    HitBlow {
        thinked: think_number,
        hit,
        blow,
    }
}

fn rand_gen() -> u8 {
    use rand::{thread_rng, Rng};

    thread_rng().gen_range(0..10)
}

fn rand_nums() -> [u8; 3] {
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
    [n1, n2, n3]
}

fn read_nums() -> [u8; 3] {
    loop {
        if let Some(nums) = read_num(readln()) {
            if nums[0] != nums[1] && nums[1] != nums[2] && nums[2] != nums[0] {
                break nums;
            }
        }
        println!("hit only 3 numbers");
    }
}
fn read_num(s: String) -> Option<[u8; 3]> {
    let mut it = s.chars();

    let n1 = char_to_num(it.next()?)?;
    let n2 = char_to_num(it.next()?)?;
    let n3 = char_to_num(it.next()?)?;

    Some([n1, n2, n3])
}

fn char_to_num(c: char) -> Option<u8> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}
