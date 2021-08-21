use std::fmt::Debug;

pub fn hello_dispatcher() {
	println!("Hello Dispatcher!");
}

pub struct Subject<T: Send + Debug> {
	subscribers: Vec<AnyObserver<T>>,
}

impl<T> Subject<T>
where
	T: Send + Debug,
{
	pub fn new() -> Self {
		Self {
			subscribers: Vec::new(),
		}
	}
}

pub trait Observable<T: Send + Debug> {
	fn next(&self, t: T);

	fn subscribe(&mut self, observer: AnyObserver<T>);
}

pub enum AnyObserver<T: Send + Debug> {
	Observer(Box<dyn Observer<T>>),
	ClojureObserver(Box<dyn Fn(&T) -> ()>),
}

impl<T: Send + Debug> Observable<T> for Subject<T> {
	fn next(&self, t: T) {
		for subscriber in &self.subscribers {
			match subscriber {
				AnyObserver::ClojureObserver(clojure) => (clojure)(&t),
				AnyObserver::Observer(ob) => (ob)(&t),
			}
		}
	}

	fn subscribe(&mut self, observer: AnyObserver<T>) {
		match observer {
			AnyObserver::Observer(t) => self.subscribers.push(AnyObserver::Observer(t)),
			AnyObserver::ClojureObserver(t) => {
				self.subscribers.push(AnyObserver::ClojureObserver(t))
			}
		};
	}
}

pub trait Observer<T: Send + Debug>: Fn(&T) -> () {
	fn receive(&self, value: &T) {
		println!("{:?}", value);
	}
}
