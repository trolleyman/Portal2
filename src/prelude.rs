pub use result::{IntoGameResult, GameResult};

pub use cg::prelude::{Angle, Array, ElementWise, EuclideanSpace, InnerSpace, Matrix, MetricSpace, One, Rotation, Rotation2, Rotation3, SquareMatrix, Transform2, Transform3, VectorSpace, Zero};
pub use cg::prelude::Transform as TransformTrait;
pub use cg::conv::*;

pub use cg::{vec2, vec3, vec4};
pub use cg::{Vector2, Vector3, Vector4};
pub use cg::{Matrix2, Matrix3, Matrix4};
pub use cg::Quaternion;
pub use cg::{Rad, Deg};
pub use cg::{Perspective, PerspectiveFov};

pub type Flt = f32;
pub type Idx = u32;

pub type Vec2 = Vector2<Flt>;
pub type Vec3 = Vector3<Flt>;
pub type Vec4 = Vector4<Flt>;

pub type IVec2 = Vector2<Idx>;
pub type IVec3 = Vector3<Idx>;
pub type IVec4 = Vector4<Idx>;

pub type Mat2 = Matrix2<Flt>;
pub type Mat3 = Matrix3<Flt>;
pub type Mat4 = Matrix4<Flt>;

pub type Quat = Quaternion<Flt>;

pub fn zero<T>() -> T where T: Zero {
	Zero::zero()
}

pub fn one<T>() -> T where T: One {
	One::one()
}
