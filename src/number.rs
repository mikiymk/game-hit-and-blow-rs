use crate::answer::HBAnswer;

pub trait HBNumber {
    type Number: PartialEq;

    fn len() -> usize;
    fn nums(&self) -> &[Self::Number];

    fn position(&self, value: &Self::Number) -> Option<usize> {
        for (i, n) in self.nums().iter().enumerate() {
            if value == n {
                return Some(i);
            }
        }
        None
    }

    fn compare(&self, other: &Self) -> HBAnswer {
        let nums = self.nums();

        let mut hit = 0;
        let mut blow = 0;
        for (i, a) in nums.into_iter().enumerate() {
            if let Some(t) = other.position(a) {
                if i == t {
                    hit += 1;
                } else {
                    blow += 1;
                }
            }
        }
        HBAnswer::new(hit, blow)
    }
}

mod three;
pub use three::Three;

fn num_to_char(iter: &mut impl Iterator<Item = char>) -> Result<u8, String> {
    iter.next()
        .ok_or(format!("reach to end"))
        .and_then(|c| match c {
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
        })
}
