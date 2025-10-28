mod math;
use math::complex::Complex;

fn main() {
let a = Complex::new(5.0, 3.0);
let b = Complex::new(2.0, 1.0);
println!("printing division: {:?}", a.div(&b));
}
