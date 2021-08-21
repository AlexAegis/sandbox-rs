use std::fmt::Debug;

use crate::{observer::AnyObserver, subscription::Subscription};

pub trait Observable<'observable, T: Send + Sized + Debug> {
	fn next(&self, t: T);

	fn subscribe<'subscription>(
		&'observable self,
		observer: AnyObserver<T>,
	) -> Subscription<'observable, T>;
}
