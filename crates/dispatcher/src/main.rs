use dispatcher::{hello_dispatcher, Observable, Subject};

pub fn main() {
	hello_dispatcher();
	let mut subject = Subject::<String>::new();
	let observer = |message: &String| println!("Got Message! {:?}", message);

	subject.subscribe(dispatcher::AnyObserver::ClojureObserver(Box::new(observer)));
	subject.subscribe(dispatcher::AnyObserver::ClojureObserver(Box::new(observer)));

	subject.subscribe(dispatcher::AnyObserver::ClojureObserver(Box::new(observer)));

	subject.next("Hello!".to_string());
	subject.next("And".to_string());
	subject.next("Welcome!".to_string());
}
