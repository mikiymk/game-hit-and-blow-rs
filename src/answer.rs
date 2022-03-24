pub struct HBAnswer {
    pub hit: u8,
    pub blow: u8,
}

impl HBAnswer {
    pub fn new(hit: u8, blow: u8) -> Self {
        HBAnswer { hit, blow }
    }

    pub fn with_match<T>(secret: &T, quest: &T) -> Self
    where
        T: crate::number::HBNumber,
    {
        secret.compare(quest)
    }

    pub fn is_end<T>(&self) -> bool
    where
        T: crate::number::HBNumber,
    {
        self.hit as usize == T::len()
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for HBAnswer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "H: {}, B: {}", self.hit, self.blow)
    }
}
