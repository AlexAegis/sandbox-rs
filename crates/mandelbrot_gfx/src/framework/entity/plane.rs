use super::vertex::Vertex;

pub struct Plane {
	vertexes: Vec<Vertex>,
	indices: Vec<u16>,
}

impl Plane {
	pub fn new(size: i8) -> Self {
		Self {
			vertexes: vec![
				Vertex::new([size, -size, 0], [0, 0, 1]),
				Vertex::new([size, size, 0], [0, 0, 1]),
				Vertex::new([-size, -size, 0], [0, 0, 1]),
				Vertex::new([-size, size, 0], [0, 0, 1]),
			],
			indices: vec![0, 1, 2, 2, 1, 3],
		}
	}
}
