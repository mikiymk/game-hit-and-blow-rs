use super::num_to_char;
use super::HBNumber;

pub struct Three<const N: usize>([u8; N]);

impl<const N: usize> Three<N> {
    pub fn new(ns: [u8; N]) -> Three<N> {
        Three(ns)
    }
}

impl<const N: usize> TryFrom<&str> for Three<N> {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Three<N>, Self::Error> {
        let mut vec = Vec::new();
        for char in value.chars() {
            vec.push(to_num(char)?);
        }

        Ok(Self::new(vec.try_into().map_err(|e| format!("{e:?}"))?))
    }
}

use std::fmt;
impl<const N: usize> fmt::Display for Three<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for n in self.nums() {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

impl<const N: usize> HBNumber for Three<N> {
    type Number = u8;
    fn len() -> usize {
        N
    }
    fn nums(&self) -> &[u8] {
        &self.0
    }
}


fn to_num(c: char) -> Result<u8, String> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        c => Err(format!("{} is not number.", c)),
    }
}
