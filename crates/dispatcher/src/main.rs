use std::sync::{Arc, Mutex};

use dispatcher::{observable::Observable, observer::AnyObserver, subject::Subject};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();
	log::debug!("Main started, env logger initialized");

	let subject = Arc::new(Mutex::new(Subject::<String>::new()));

	let _sub1 = subject
		.lock()
		.unwrap()
		.subscribe(AnyObserver::ClojureObserver(Box::new(
			|message: &String| println!("Got Message in first subscription! {:?}", message),
		)));

	let _sub2 = subject
		.lock()
		.unwrap()
		.subscribe(AnyObserver::ClojureObserver(Box::new(
			|message: &String| println!("Got Message in second subscription! {:?}", message),
		)));

	let s1 = subject.clone();
	let h1 = std::thread::spawn(move || {
		std::thread::sleep(std::time::Duration::from_secs(1));
		s1.lock().unwrap().next("Welcome from thread 1".to_string());
		// sub1.unsubscribe();
		return s1;
	});

	let s2 = subject.clone();
	let h2 = std::thread::spawn(move || {
		std::thread::sleep(std::time::Duration::from_secs(2));
		s2.lock().unwrap().next("Welcome from thread 2".to_string());
		return s2;
	});

	h1.join().unwrap();
	h2.join().unwrap();

	Ok(())
}
