use crate::vec::Vec3;
use num::Float;
use num_traits::NumAssign;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle<T: Float> {
    pub position: Vec3<T>,
    pub velocity: Vec3<T>,
    pub acceleration: Vec3<T>,
    pub damping: T,
    /// We store the inverse mass because it makes infinite mass possible and zero mass impossible.
    ///
    /// Zero mass would be problematic because any force would result in infinite acceleration,
    /// and more practically, it would result in division by zero.
    ///
    /// Infinite mass is a convenient way to make an immovable object.
    ///
    /// Inverse mass is also conveniently used in our physics equations:
    /// ```text
    /// f = ma  ∴  a = f*(1/m)
    /// ```
    /// `(1/m)` is inverse mass.
    pub inverse_mass: T,
}

impl<T: Float + NumAssign> Particle<T> {
    pub fn integrate(&mut self, duration: T) {
        self.position += self.velocity * duration;
        self.velocity += self.acceleration * duration;
        self.velocity *= self.damping.powf(duration);
    }
}
