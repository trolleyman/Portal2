use prelude::*;

#[derive(Debug, Clone)]
pub struct Camera {
	pub fovy: Rad<Flt>,
	pub pos: Vec3,
	pub angx: Rad<Flt>,
	pub angy: Rad<Flt>,
	pub znear: Flt,
	pub zfar: Flt,
}
impl Default for Camera {
	fn default() -> Camera {
		Camera {
			fovy: Rad::from(Deg(60.0)),
			pos: zero(),
			angx: zero(),
			angy: zero(),
			znear: 0.01,
			zfar: 1000.0,
		}
	}
}
impl Camera {
	pub fn view_matrix(&self) -> Mat4 {
		// 1. Translation
		let trans = Mat4::from_translation(-self.pos);
		// 2. Rotation (x).
		// This is confusing because it uses from_angle_y.
		// This is because the x rotation is /around/ the y axis. 
		let rot_x = Mat4::from(Quaternion::from_angle_y(self.angx));
		// 3. Rotation (y)
		// The same principle applies here.
		let rot_y = Mat4::from(Quaternion::from_angle_x(self.angy));
		// 3 * 2 * 1 because matrices multiply backwards
		(rot_y * rot_x * trans)
	}
	
	pub fn projection_matrix(&self, w: u32, h: u32) -> Mat4 {
		Mat4::from(PerspectiveFov {
			fovy: self.fovy,
			aspect: w as Flt / h as Flt,
			near: self.znear,
			far: self.zfar,
		})
	}
	
	pub fn move_camera(&mut self, v: Vec3) {
		// Take account of the rotation of the camera
		let rot = Mat4::from_angle_y(-self.angx);
		self.pos += (rot * v.extend(0.0)).truncate();
	}
	
	pub fn rotate_player(&mut self, r: Vector2<Rad<Flt>>) {
		self.angx += r.x;
		self.angy += r.y;
	}
}
