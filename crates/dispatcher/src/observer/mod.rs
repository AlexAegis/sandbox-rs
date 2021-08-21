pub mod logging_observer;

use std::fmt::Debug;

pub enum AnyObserver<T: Send + Sized + Debug> {
	Observer(Box<dyn Observer<T> + Send>),
	ClojureObserver(Box<dyn Fn(&T) -> () + Send>),
}

pub trait Observer<T: Send + Sized + Debug> {
	fn next(&self, value: &T);
}
