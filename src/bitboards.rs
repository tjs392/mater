/// bitboards.rs handles bitboard function utils


// sets bit in a bitboard
pub fn set_bit(bb: &mut u64, sq: usize) {
    *bb |= 1u64 << sq;
}

// sets a bit to 0 in bitboard
pub fn clear_bit(bb: &mut u64, sq: usize) {
    *bb &= !(1u64 << sq);
}

// gets bit at square
pub fn get_bit(bb: &u64, sq: usize) -> bool {
    (bb & (1u64 << sq)) != 0
}

// returns the index of the LSB that == 1
// then clears that bit from the bitboard
pub fn next_bit(bb: &mut u64) -> usize {
    let lsb_index = bb.trailing_zeros() as usize;
    *bb &= *bb - 1;
    lsb_index 
}