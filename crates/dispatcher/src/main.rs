use dispatcher::{Observable, Observer, Subject};

struct SimpleObserver<T: std::fmt::Debug> {
	id: T,
}

impl<T: std::fmt::Debug> SimpleObserver<T> {
	fn new(id: T) -> Self {
		Self { id }
	}
}

impl<T: Send + Sync + std::fmt::Debug> Observer<T> for SimpleObserver<T> {
	fn next(&self, value: &T) {
		println!("SimpleObserver {:?} got value: {:?}", self.id, value);
	}
}

pub fn main() {
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();
	log::debug!("Main started, env logger initialized");

	let subject = Subject::<String>::new();
	let simple_observer = SimpleObserver::new("s1".to_string());

	subject.next("No one will hear me!".to_string());

	let simple_subscription =
		subject.subscribe(dispatcher::AnyObserver::Observer(Box::new(simple_observer)));

	subject.subscribe(dispatcher::AnyObserver::ClojureObserver(Box::new(
		|message: &String| println!("Got Message in first subscription! {:?}", message),
	)));
	subject.subscribe(dispatcher::AnyObserver::ClojureObserver(Box::new(
		|message: &String| println!("Got Message in second subscription! {:?}", message),
	)));

	let welcoming_subscription = subject.subscribe(dispatcher::AnyObserver::ClojureObserver(
		Box::new(|message: &String| println!("Got Message in third subscription! {:?}", message)),
	));

	subject.next("Hello!".to_string());
	simple_subscription.unsubscribe();

	subject.next("And".to_string());
	welcoming_subscription.unsubscribe();
	subject.next("Welcome!".to_string());
}
