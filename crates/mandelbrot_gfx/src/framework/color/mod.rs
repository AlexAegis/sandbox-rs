use cgmath::Vector3;
use wgpu::Color;

#[derive(Debug)]
pub struct VectorColor(pub Color);

impl From<VectorColor> for Vector3<f64> {
	fn from(color: VectorColor) -> Self {
		Self {
			x: color.0.r,
			y: color.0.g,
			z: color.0.b,
		}
	}
}

impl From<Color> for VectorColor {
	fn from(color: Color) -> Self {
		VectorColor(color)
	}
}

impl From<Vector3<f64>> for VectorColor {
	fn from(color: Vector3<f64>) -> Self {
		VectorColor(Color {
			r: color.x,
			g: color.y,
			b: color.z,
			a: 1.0,
		})
	}
}
