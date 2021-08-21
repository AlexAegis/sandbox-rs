use crate::subject::Subject;
use std::fmt::Debug;

pub struct Subscription<'subscription, T: Send + Sized + Debug> {
	observer: &'subscription Subject<'subscription, T>,
	index: usize,
}

impl<'subscription, T: Send + Sized + Debug> Subscription<'subscription, T> {
	pub fn new(
		observer: &'subscription Subject<'subscription, T>,
		index: usize,
	) -> Subscription<'subscription, T> {
		Subscription { observer, index }
	}

	pub fn unsubscribe(self) {
		self.observer.remove_subscriber(self.index)
	}
}
