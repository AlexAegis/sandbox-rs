use super::Observer;

pub struct LoggingObserver<T: std::fmt::Debug> {
	id: T,
}

impl<T: std::fmt::Debug> LoggingObserver<T> {
	pub fn new(id: T) -> Self {
		Self { id }
	}
}

impl<T: Send + Sync + std::fmt::Debug> Observer<T> for LoggingObserver<T> {
	fn next(&self, value: &T) {
		log::info!("SimpleObserver {:?} got value: {:?}", self.id, value);
	}
}
