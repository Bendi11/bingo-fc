pub mod vec;
pub mod quat;

pub use vec::Vector3;
pub use quat::Quaternion;

pub trait ClosedMul: core::ops::Mul<Output = Self> + Sized {}
pub trait ClosedAdd: core::ops::Add<Output = Self> + Sized {}

impl<T> ClosedMul for T
where T: core::ops::Mul<Output = T> + Sized {}
impl<T> ClosedAdd for T
where T: core::ops::Add<Output = T> + Sized {}
