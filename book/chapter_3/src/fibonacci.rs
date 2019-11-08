use num::{One, Zero};
use std::cmp::PartialEq;
use std::ops::{Add, Sub};

pub fn fibonacci_recursive(n: i32) -> i32 {
	match n {
		0 => 0,
		1 => 1,
		n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
	}
}

pub fn fibonacci_recursive_generic<
	T: Copy + One + Zero + Add<Output = T> + Sub<Output = T> + PartialEq,
>(
	n: T,
) -> T {
	if T::is_zero(&n) {
		T::zero()
	} else if T::is_one(&n) {
		T::one()
	} else {
		let nm = n - T::one();
		fibonacci_recursive_generic::<T>(nm) + fibonacci_recursive_generic::<T>(nm - T::one())
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	#[test]
	fn fibonacci_recursive_test() {
		assert_eq!(fibonacci_recursive(6), 8);
		assert_eq!(fibonacci_recursive(2), 1);
	}

	#[test]
	fn fibonacci_recursive_generic_test() {
		assert_eq!(fibonacci_recursive_generic::<i32>(6), 8);
		assert_eq!(fibonacci_recursive_generic::<i32>(2), 1);
	}
}
