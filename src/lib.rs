use num::Float;

// We may want to derive an Eq implementation for Vec3,
// but we don't have a reason to (for now). It is better
// to avoid committing to that interface until later.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float>(T, T, T);

impl<T: Float> Vec3<T> {
    pub fn invert(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn invert() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v.invert(), Vec3(-1.0, -2.0, -3.0));
    }
}
