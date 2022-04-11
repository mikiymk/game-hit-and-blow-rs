use super::num_to_char;
use super::HBNumber;

pub struct Three<const N: usize>([u8; N]);

impl<const N: usize> Three<N> {
    pub fn new(ns: [u8; N]) -> Three<N> {
        Three(ns)
    }
}

impl<const N: usize> TryFrom<&str> for Three<N> {
    type Error = String;
    fn try_from(value: &str) -> Result<Three<N>, Self::Error> {
        let mut vec = Vec::new();
        for char in value.chars() {
            vec.push(char.parse()?);
        }

        Ok(Self::new(vec.try_into()?))
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
