use std::ops::{Add, Div, Mul, Sub};

/// (F − 32) × 5/9
pub fn f_to_c<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T>>(f: T) -> T
where
	f32: Into<T>,
{
	(f - (32.).into()) * (5.).into() / (9.).into()
}

/// (C × 9/5) + 32
pub fn c_to_f<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T>>(c: T) -> T
where
	f32: Into<T>,
{
	(c * (9.).into() / (5.).into()) + (32.).into()
}

#[cfg(test)]
mod tests {

	use super::*;
	use float_cmp::ApproxEq;

	#[test]
	fn c_to_f_test() {
		assert!(c_to_f::<f64>(6.0_f64).approx_eq(42.8_f64, (0.0, 2)));

		let a: u8;
	}

	#[test]
	fn f_to_c_test() {
		assert!(f_to_c::<f64>(42.8_f64).approx_eq(6.0_f64, (0.0, 2)));
	}
}
