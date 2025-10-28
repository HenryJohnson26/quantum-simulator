pub fn get_bit(index: usize, bit: usize) -> bool {
    (index >> bit) & 1 == 1
}

pub fn flip_bit(index: usize, bit: usize) -> usize {
    index ^ (1 << bit)
}
