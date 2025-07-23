use nalgebra::{Matrix3x4, Quaternion, UnitQuaternion, UnitVector3, Vector3, VectorView3};



pub struct MadgwickAhrs {
    q: UnitQuaternion<f32>,
    four: f32,
    half: f32,
}

impl MadgwickAhrs
{
    pub fn update(&mut self, gyro: Vector3<f32>, accel: VectorView3<f32>, deltat: f32) {
        let q_rates = Quaternion::from_imag(gyro) * self.q;
        //let grad = Self::objective_jacobian(self.q).transpose() * Self::objective(self.q, accel);
        //let step = grad.normalize();
        //self.q = self.q + (q_rates - self.beta * grad) * deltat;
    }

    fn objective(q: Quaternion<f32>, a: UnitVector3<f32>) -> Vector3<f32> {
        Vector3::new(
            2f32*(q.i*q.k - q.w*q.j) - a.x,
            2f32*(q.w*q.i + q.j*q.k) - a.y,
            2f32*(0.5f32 - q.i*q.i -q.j*q.j) - a.z
        )
    }

    fn objective_jacobian(&self, q: Quaternion<f32>) -> Matrix3x4<f32> {
        return Matrix3x4::new(
            -self.two*q.j, self.two*q.k, -self.two*q.w, self.two*q.i,
            self.two*q.i, self.two*q.w, self.two*q.k, self.two*q.j,
            T::zero(), -self.four*q.i, -self.four*q.j, T::zero()
        )
    }

}
