use super::num_to_char;
use super::HBNumber;

pub struct Three([u8; 3]);

impl Three {
    pub fn new(n: u8, m: u8, l: u8) -> Three {
        Three([n, m, l])
    }
}

impl TryFrom<&str> for Three {
    type Error = String;
    fn try_from(value: &str) -> Result<Three, Self::Error> {
        let mut iter = value.chars();
        let num1 = num_to_char(&mut iter)?;
        let num2 = num_to_char(&mut iter)?;
        let num3 = num_to_char(&mut iter)?;

        match iter.next() {
            Some(c) => Err(format!("unnecessary char {}", c)),
            None => Ok(Self::new(num1, num2, num3)),
        }
    }
}

use std::fmt;
impl fmt::Display for Three {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for n in self.nums() {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

impl HBNumber for Three {
    type Number = u8;
    fn len() -> usize {
        3
    }
    fn nums(&self) -> &[u8] {
        &self.0
    }
}
