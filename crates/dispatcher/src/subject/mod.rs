use std::{
	collections::HashMap,
	fmt::Debug,
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use crate::{observable::Observable, observer::AnyObserver, subscription::Subscription};

pub struct Subject<'subject, T: Send + Debug + Sized> {
	index: Arc<Mutex<usize>>,
	subscribers: Arc<Mutex<HashMap<usize, AnyObserver<T>>>>,
	phantom: PhantomData<&'subject T>,
}

impl<'subject, T> Subject<'subject, T>
where
	T: Send + Sized + Debug,
{
	pub fn new() -> Self {
		log::debug!("Created Subject!");
		Self {
			phantom: PhantomData {},
			index: Arc::new(Mutex::new(0)),
			subscribers: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	pub fn remove_subscriber(&self, index: usize) {
		log::debug!("Unsubscribe index: {}", index);
		self.subscribers.lock().unwrap().remove_entry(&index);
	}
}

impl<'observable, T: Send + Sized + Debug> Observable<'observable, T> for Subject<'observable, T> {
	fn next(&self, t: T) {
		log::debug!("Notifying subscribers about {:?}", t);
		for subscriber in self.subscribers.lock().unwrap().values().into_iter() {
			match subscriber {
				AnyObserver::ClojureObserver(clojure) => (clojure)(&t),
				AnyObserver::Observer(ob) => ob.next(&t),
			}
		}
		log::debug!("Notified all subscribers about {:?}", t);
	}

	fn subscribe<'subscription>(
		&'observable self,
		observer: AnyObserver<T>,
	) -> Subscription<'observable, T> {
		let current_index: usize = *self.index.lock().unwrap();
		log::debug!("Subscribe to index {}", current_index);

		*self.index.lock().unwrap() += 1;

		self.subscribers
			.lock()
			.unwrap()
			.insert(current_index, observer);

		Subscription::new(self, current_index)
	}
}
