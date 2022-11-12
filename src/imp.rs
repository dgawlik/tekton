
pub mod b128;

pub mod b256;

pub mod util;

pub mod primitives;

#[derive(PartialEq)]
pub enum Rounds {
    FASTER,
    SAFER
}


pub struct Flags {
    pub rounds: Rounds,
}