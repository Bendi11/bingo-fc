use nalgebra::VectorView3;



pub trait Accelerometer<T> {
    fn linear_acceleration(&mut self) -> VectorView3<T>;
}
