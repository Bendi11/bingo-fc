use nalgebra::{Matrix3x4, Quaternion, Vector3};
use nalgebra as na;


pub struct MadgwickAhrs<T> {
    q: Quaternion<T>,
    four: T,
    half: T,
    two: T,
    beta: T,
}

impl<T> MadgwickAhrs<T>
    where T: na::RealField + Copy
{
    pub fn new(beta: T) -> Self {
        let two = T::one() + T::one();
        Self {
            q: Quaternion::new(T::zero(), T::zero(), T::zero(), T::one()),
            four: two + two,
            half: T::one() / two,
            two,
            beta,
        }
    }

    pub const fn quat(&self) -> &Quaternion<T> {
        &self.q
    }

    pub fn update(&mut self, gyro: Vector3<T>, accel: Vector3<T>, deltat: T) {
        let q_rates = Quaternion::from_imag(gyro) * self.q; 
        let grad = self.objective_jacobian(self.q).transpose() * self.objective(self.q, accel);
        let step = grad.normalize();
        self.q += (q_rates - Quaternion::from_vector(step * self.beta)) * deltat;
        self.q = self.q.normalize();
    }

    fn objective(&self, q: Quaternion<T>, a: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.two*(q.i*q.k - q.w*q.j) - a.x,
            self.two*(q.w*q.i + q.j*q.k) - a.y,
            self.two*(self.half - q.i*q.i -q.j*q.j) - a.z
        )
    }

    fn objective_jacobian(&self, q: Quaternion<T>) -> Matrix3x4<T> {
        Matrix3x4::new(
            -self.two*q.j, self.two*q.k, -self.two*q.w, self.two*q.i,
            self.two*q.i, self.two*q.w, self.two*q.k, self.two*q.j,
            T::zero(), -self.four*q.i, -self.four*q.j, T::zero()
        )
    }

}

impl<T> Default for MadgwickAhrs<T>
where T: na::RealField + Copy
{
    fn default() -> Self {
        Self::new(T::one())
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::UnitQuaternion;

    use super::*;

    #[test]
    fn test_madgwick() {
        let R2D: fn(f32) -> f32 = |v| v * 180f32 / std::f32::consts::PI;
        let mut filter = MadgwickAhrs::<f32>::default();
        let gravity = Vector3::new(1f32, 1f32, 0f32).normalize();
        for i in 0..100000 {
            filter.update(Vector3::new(0f32, 0f32, 0f32), gravity, 1f32 / 1000f32);
            let (roll, pitch, yaw) = UnitQuaternion::from_quaternion(*filter.quat()).euler_angles();
            let (roll, pitch, yaw) = (R2D(roll), R2D(pitch), R2D(yaw));
            let transformed = UnitQuaternion::from_quaternion(*filter.quat()).transform_vector(&gravity);
            println!("i{i:3} - r{roll:4.2} p{pitch:4.2} y{yaw:4.2} - {transformed}");
        }
        panic!()
    }

}
