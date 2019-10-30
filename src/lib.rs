use num::Float;
use num_traits::NumAssign;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// We may want to derive an Eq implementation for Vec3,
// but we don't have a reason to (for now). It is better
// to avoid committing to that interface until later.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float>(pub T, pub T, pub T);

impl<T: Float + NumAssign> Vec3<T> {
    pub fn invert(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }

    pub fn mag(self) -> T {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    /// `vec.mag_squared()` will avoid a `sqrt` call that `vec.mag().powi(2)` would make.
    pub fn mag_squared(self) -> T {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn norm(self) -> Self {
        let mag = self.mag();
        if mag.is_zero() {
            self
        } else {
            self / mag
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn basis(self, other: Self) -> Option<(Self, Self, Self)> {
        let a = self;
        let b = other;
        let c = a.norm().cross(b).norm();
        if c.mag().is_zero() {
            None
        } else {
            Some((a, c.cross(a), c))
        }
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<T: Float + NumAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<T: Float + NumAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl<T: Float> Mul for Vec3<T> {
    type Output = Self;

    /// Component Product
    /// ```
    /// # use cyclone::Vec3;
    /// let v1 = Vec3(1.0, 2.0, 3.0);
    /// let v2 = Vec3(2.0, 2.0, 2.0);
    /// assert_eq!(v1 * v2, Vec3(2.0, 4.0, 6.0));
    /// ```
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<T: Float + NumAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl<T: Float + NumAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, other: T) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl<T: Float + NumAssign> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, other: T) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn invert() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v.invert(), Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn mag() {
        assert_approx_eq!(Vec3(0.0, 2.0, 0.0).mag(), 2.0_f64);
        assert_approx_eq!(Vec3(3.0, 1.0, 1.0).mag(), 3.316_624_790_f64);
    }

    #[test]
    fn mag_squared() {
        assert_approx_eq!(Vec3(0.0, 2.0, 0.0).mag_squared(), 4.0_f64);
        assert_approx_eq!(Vec3(3.0, 1.0, 1.0).mag_squared(), 11.0_f64);
    }

    #[test]
    fn norm() {
        assert_eq!(Vec3(0.0, 2.0, 0.0).norm(), Vec3(0.0, 1.0, 0.0));
        let v = Vec3(3.0, 1.0, 1.0);
        let v_norm = v.norm();
        assert_approx_eq!(v_norm.0, 0.904_534_034_f64);
        assert_approx_eq!(v_norm.1, 0.301_511_345_f64);
        assert_approx_eq!(v_norm.2, 0.301_511_345_f64);
        assert_eq!(Vec3(0.0, 0.0, 0.0).norm(), Vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn dot() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, -5.0, 6.0);
        assert_eq!(a.dot(b), 12.0);
    }

    #[test]
    fn cross() {
        let a = Vec3(2.0, 3.0, 4.0);
        let b = Vec3(5.0, 6.0, 7.0);
        assert_eq!(a.cross(b), Vec3(-3.0, 6.0, -3.0));
    }

    #[test]
    fn basis() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(1.0, 99.0, 0.0);
        let (basis_x, basis_y, basis_z) = a.basis(b).unwrap();
        assert_eq!(basis_x, Vec3(1.0, 0.0, 0.0));
        assert_eq!(basis_y, Vec3(0.0, 1.0, 0.0));
        assert_eq!(basis_z, Vec3(0.0, 0.0, 1.0));
        assert_eq!(a.basis(a), None);
    }

    #[test]
    fn ops() {
        let a = Vec3(1.0, 1.0, 1.0);
        let b = Vec3(0.0, 1.0, 2.0);
        assert_eq!(a + b, Vec3(1.0, 2.0, 3.0));
        assert_eq!(a - b, Vec3(1.0, 0.0, -1.0));
        assert_eq!(a * b, Vec3(0.0, 1.0, 2.0));
        assert_eq!(a * 2.0, Vec3(2.0, 2.0, 2.0));
        assert_eq!(a / 2.0, Vec3(0.5, 0.5, 0.5));
        let mut add = a;
        let mut sub = a;
        let mut mul = a;
        let mut scale = a;
        let mut div = a;
        add += b;
        sub -= b;
        mul *= b;
        scale *= 2.0;
        div /= 2.0;
        assert_eq!(add, Vec3(1.0, 2.0, 3.0));
        assert_eq!(sub, Vec3(1.0, 0.0, -1.0));
        assert_eq!(mul, Vec3(0.0, 1.0, 2.0));
        assert_eq!(scale, Vec3(2.0, 2.0, 2.0));
        assert_eq!(div, Vec3(0.5, 0.5, 0.5));
    }
}
