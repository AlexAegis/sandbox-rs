use num::{One, Zero};
use std::cmp::{Ord, PartialEq};
use std::fmt::Debug;
use std::ops::{Add, Sub};

pub fn fibonacci_recursive<T: Copy + One + Zero + Add<Output = T> + Sub<Output = T> + PartialEq>(
	n: T,
) -> T {
	if T::is_zero(&n) {
		T::zero()
	} else if T::is_one(&n) {
		T::one()
	} else {
		let nm = n - T::one();
		fibonacci_recursive::<T>(nm) + fibonacci_recursive::<T>(nm - T::one())
	}
}

pub fn fibonacci<T: Debug + Copy + Ord + One + Zero + Add<Output = T>>(n: usize) -> T {
	let mut v: Vec<T> = vec![T::zero(), T::one()];
	for i in 1..n {
		v.push(*v.get(i).unwrap() + *v.get(i - 1).unwrap());
	}
	*v.get(n).unwrap()
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn fibonacci_test() {
		assert_eq!(fibonacci::<i32>(0), 0);
		assert_eq!(fibonacci::<i32>(1), 1);
		assert_eq!(fibonacci::<i32>(2), 1);
		assert_eq!(fibonacci::<i32>(3), 2);
		assert_eq!(fibonacci::<i32>(4), 3);
		assert_eq!(fibonacci::<i32>(5), 5);
		assert_eq!(fibonacci::<i32>(6), 8);
		assert_eq!(fibonacci::<i32>(7), 13);
		assert_eq!(fibonacci::<i64>(92), 7_540_113_804_746_346_429);
	}

	#[test]
	fn fibonacci_recursive_test() {
		assert_eq!(fibonacci_recursive(6), 8);
		assert_eq!(fibonacci_recursive(2), 1);
	}
}
