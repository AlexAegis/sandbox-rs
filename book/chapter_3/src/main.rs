pub mod christmas;
pub mod fibonacci;
pub mod temperature;

pub fn main() {
	println!(
		"Temperature 28 C° to F° is {:?}, 70 F° to C° is {:?}",
		temperature::c_to_f(28.),
		temperature::f_to_c::<f64>(28.0_f64)
	);
	println!(
		"Fibonacci recursive, 6th number is {:?}",
		fibonacci::fibonacci_recursive(6)
	);
	println!(
		"Fibonacci iterative, 92th number is {:?}",
		fibonacci::fibonacci::<i64>(92)
	);
	println!("{}", christmas::christmas());
}
