pub fn xor_row(dst: &mut [bool], src: &[bool]) {
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d ^= *s;
    }
}
