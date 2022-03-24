use crate::number::HBNumber;

pub trait Player<T>
where
    T: HBNumber,
{
    fn get_number(&mut self) -> T;
}
