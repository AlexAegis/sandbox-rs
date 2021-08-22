use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
	_pos: [i8; 4],
	_normal: [i8; 4],
}

impl Vertex {
	pub fn new(pos: [i8; 3], nor: [i8; 3]) -> Vertex {
		Vertex {
			_pos: [pos[0], pos[1], pos[2], 1],
			_normal: [nor[0], nor[1], nor[2], 0],
		}
	}
}
