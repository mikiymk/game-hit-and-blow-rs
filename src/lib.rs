use std::fmt::{Display, Formatter, Result};

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
        if hb.is_end() {
            println!("you win");
            break;
        }

        let think_number = rand_nums();
        let hb = hit_and_blow(&your_number, &think_number);
        println!("I:{} {}", think_number, hb);
        if hb.is_end() {
            println!("I win");
            break;
        }
    }
}

#[derive(PartialEq)]
struct Numbers(u8, u8, u8);

impl Numbers {
    fn new(i: u8, j: u8, k: u8) -> Numbers {
        Numbers(i, j, k)
    }

    fn position(&self, value: u8) -> Option<usize> {
        match value {
            i if self.0 == i => Some(0),
            i if self.1 == i => Some(1),
            i if self.2 == i => Some(2),
            _ => None,
        }
    }

    fn iter(&self) -> impl IntoIterator<Item = u8> {
        [self.0, self.1, self.2]
    }
}

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

struct HitBlow {
    hit: u8,
    blow: u8,
}

impl HitBlow {
    fn new(hit: u8, blow: u8) -> Self {
        HitBlow { hit, blow }
    }
    fn is_end(&self) -> bool {
        self.hit == 3
    }
}

impl Display for HitBlow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "H: {}, B: {}", self.hit, self.blow)
    }
}

fn hit_and_blow(act_number: &Numbers, think_number: &Numbers) -> HitBlow {
    let mut hit = 0;
    let mut blow = 0;
    for (i, a) in act_number.iter().into_iter().enumerate() {
        if let Some(t) = think_number.position(a) {
            if i == t {
                hit += 1;
            } else {
                blow += 1;
            }
        }
    }

    HitBlow::new(hit, blow)
}

fn rand_gen() -> u8 {
    use rand::{thread_rng, Rng};

    thread_rng().gen_range(0..10)
}

fn rand_nums() -> Numbers {
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
    Numbers::new(n1, n2, n3)
}

fn read_nums() -> Numbers {
    loop {
        if let Some([x, y, z]) = read_num(readln()) {
            if x != y && y != z && z != x {
                break Numbers(x, y, z);
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
