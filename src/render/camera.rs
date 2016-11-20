use prelude::*;

use world::entity::Portal;

#[derive(Debug, Copy, Clone)]
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
	
	pub fn transform_by_portal(&mut self, p_from: Portal, p_to: Portal) {
		let rot_x = p_to.angx - p_from.angx;
		let rot_y = p_to.angy - p_from.angy;
		self.angx += rot_x;
		self.angy += rot_y;
		// v = the vector from p_from to the camera
		let mut v = (self.pos - p_from.pos).extend(0.0);
		// rotate v by rot_x
		v = Mat4::from_angle_y(rot_x + Rad::from(Deg(180.0))) * v;
		// rotate v by rot_y
		v = Mat4::from_angle_x(rot_y) * v;
		// add v to p_to's position
		self.pos = v.truncate() + p_to.pos;
	}
}
