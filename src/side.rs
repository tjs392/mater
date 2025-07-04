// side.rs handles Side enum and util funcs

impl From<usize> for Side {
    fn from(val: usize) -> Self {
        match val {
            0 => Side::White,
            1 => Side::Black,
            _ => panic!("Invalid side index: {}", val),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Side {
    White = 0,
    Black = 1,
}

pub const SIDES: usize = 2;
