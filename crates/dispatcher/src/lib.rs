use std::{
	collections::HashMap,
	fmt::Debug,
	marker::PhantomData,
	sync::{Arc, Mutex},
};

pub fn hello_dispatcher() {
	println!("Hello Dispatcher!");
}

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

	fn unsubscribe(&self, index: usize) {
		log::debug!("Unsubscribe index: {}", index);
		self.subscribers.lock().unwrap().remove_entry(&index);
	}
}

pub trait Observable<'observable, T: Send + Sized + Debug> {
	fn next(&self, t: T);

	fn subscribe<'subscription>(
		&'observable self,
		observer: AnyObserver<T>,
	) -> Subscription<'observable, T>;
}

pub enum AnyObserver<T: Send + Sized + Debug> {
	Observer(Box<dyn Observer<T>>),
	ClojureObserver(Box<dyn Fn(&T) -> ()>),
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

pub trait Observer<T: Send + Sized + Debug> {
	fn next(&self, value: &T);
}

pub struct Subscription<'subscription, T: Send + Sized + Debug> {
	observer: &'subscription Subject<'subscription, T>,
	index: usize,
	phantom: PhantomData<&'subscription i32>,
}

impl<'subscription, T: Send + Sized + Debug> Subscription<'subscription, T> {
	fn new(
		observer: &'subscription Subject<'subscription, T>,
		index: usize,
	) -> Subscription<'subscription, T> {
		Subscription {
			observer,
			index,
			phantom: PhantomData {},
		}
	}

	pub fn unsubscribe(self) {
		self.observer.unsubscribe(self.index)
	}
}
