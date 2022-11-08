
pub mod b128;

pub mod b256;

pub mod util;

pub mod primitives;

pub enum Permute {
    PERMUTE,
    ROTATE
}


pub enum Mode {
    BYTE,
    INT
}


pub struct Flags {
    pub permute: Permute,
    pub mode: Mode
}