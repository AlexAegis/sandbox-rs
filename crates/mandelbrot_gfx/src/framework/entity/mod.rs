use bytemuck::{Pod, Zeroable};

pub mod cube;
pub mod plane;
pub mod vertex;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct EntityUniforms {
	color: [f32; 4],
}
