extern crate num;

mod fibonacci;

pub fn main() {
	println!("Fibonacci {:?}", fibonacci::fibonacci_recursive(6));
}
