use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Debug for Vec3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn splat(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn with_x(self, x: f32) -> Vec3 {
        return Vec3 {
            x: x,
            y: self.y,
            z: self.z,
        };
    }

    pub fn with_y(self, y: f32) -> Vec3 {
        return Vec3 {
            x: self.x,
            y: y,
            z: self.z,
        };
    }

    pub fn with_z(self, z: f32) -> Vec3 {
        return Vec3 {
            x: self.x,
            y: self.y,
            z: z,
        };
    }

    pub fn normalize(self) -> Vec3 {
        self / self.len()
    }

    pub fn len(&self) -> f32 {
        return self.len_squared().sqrt();
    }

    pub fn min(&self, rhs: &Vec3) -> Vec3 {
        return Vec3 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        };
    }

    pub fn max(&self, rhs: &Vec3) -> Vec3 {
        return Vec3 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        };
    }

    pub fn len_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        return Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        };
    }
}

macro_rules! impl_binary_operations {
  // $VectorType -> `Vec3`
  // $Trait -> `Add`, $binary_fn -> `add`, $binary_symbol -> `+`
  ($VectorType:ident $Trait:ident $binary_fn:ident $binary_symbol:tt) => {
    // All other implementations forward through to this implementation
    // a: &$VectorType, b: &$VectorType
    impl<'a, 'b> $Trait<&'a $VectorType> for &'b $VectorType {
      type Output = $VectorType;
      fn $binary_fn(self, rhs: &'a $VectorType) -> $VectorType {
        $VectorType {
          x: self.x $binary_symbol rhs.x,
          y: self.y $binary_symbol rhs.y,
          z: self.z $binary_symbol rhs.z,
        }
      }
    }

    // a: $VectorType, b: $VectorType
    impl $Trait<$VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self, rhs: $VectorType) -> $VectorType {
        &self $binary_symbol &rhs
      }
    }

    // a: $VectorType, b: &$VectorType
    impl<'a> $Trait<&'a $VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self, rhs: &'a $VectorType) -> $VectorType {
        &self $binary_symbol rhs
      }
    }

    // a: &$VectorType, b: $VectorType
    impl<'a> $Trait<$VectorType> for &'a $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self, rhs: $VectorType) -> $VectorType {
        self $binary_symbol &rhs
      }
    }

    impl<'a> $Trait<f32> for &'a $VectorType {
      type Output = $VectorType;

      fn $binary_fn(self, rhs: f32) -> $VectorType {
        $VectorType {
          x: self.x $binary_symbol rhs,
          y: self.y $binary_symbol rhs,
          z: self.z $binary_symbol rhs
        }
      }
    }

    impl $Trait<f32> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self, rhs: f32) -> $VectorType {
        &self $binary_symbol rhs
      }
    }

    impl $Trait<$VectorType> for f32 {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self, rhs: $VectorType) -> $VectorType {
        &rhs $binary_symbol self
      }
    }

    impl<'a> $Trait<&'a $VectorType> for f32 {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self, rhs: &'a $VectorType) -> $VectorType {
        rhs $binary_symbol self
      }
    }
  };
}

macro_rules! impl_unary_operations {
  ($VectorType:ident $Trait:ident $binary_fn:ident $binary_symbol:tt) => {

    impl<'a> $Trait for &'a $VectorType {
      type Output = $VectorType;

      fn $binary_fn(self) -> Vec3 {
        $VectorType {
          x: $binary_symbol self.x,
          y: $binary_symbol self.y,
          z: $binary_symbol self.z,
        }
      }
    }

    impl $Trait for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $binary_fn(self) -> Vec3 {
        $binary_symbol &self
      }
    }
  };
}

macro_rules! impl_op_assign {
  ($VectorType:ident $TraitAssign:ident $binary_fn:ident $binary_symbol:tt) => {

    impl<'a> $TraitAssign<&'a $VectorType> for $VectorType {
      fn $binary_fn(&mut self, rhs: &'a $VectorType) {
        *self = $VectorType {
          x: self.x $binary_symbol rhs.x,
          y: self.y $binary_symbol rhs.y,
          z: self.z $binary_symbol rhs.z,
        };
      }
    }

    impl $TraitAssign for $VectorType {
      #[inline]
      fn $binary_fn(&mut self, rhs: $VectorType) {
        *self = *self $binary_symbol &rhs
      }
    }
  };
}

impl_binary_operations!(Vec3 Add add +);
impl_binary_operations!(Vec3 Sub sub -);
impl_binary_operations!(Vec3 Mul mul *);
impl_binary_operations!(Vec3 Div div /);

impl_op_assign!(Vec3 AddAssign add_assign +);
impl_op_assign!(Vec3 SubAssign sub_assign -);
impl_op_assign!(Vec3 MulAssign mul_assign *);
impl_op_assign!(Vec3 DivAssign div_assign /);

impl_unary_operations!(Vec3 Neg neg -);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(
            Vec3::new(0.0, 1.0, 2.0) + Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(3.0, 5.0, 7.0)
        );
        assert_eq!(a + Vec3::new(3.0, 4.0, 5.0), Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + Vec3::new(3.0, 4.0, 5.0), Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + b, Vec3::new(3.0, 5.0, 7.0));

        // Test for RHS value type
        {
            let mut c = Vec3::ONE;
            c += a;
            assert_eq!(c, Vec3::new(1.0, 2.0, 3.0));
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::ONE;
            c += &a;
            assert_eq!(c, Vec3::new(1.0, 2.0, 3.0));
        }
    }

    #[test]
    fn subtract() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(
            Vec3::new(0.0, 1.0, 2.0) - Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(-3.0, -3.0, -3.0)
        );
        assert_eq!(a - Vec3::new(3.0, 4.0, 5.0), Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(&a - Vec3::new(3.0, 4.0, 5.0), Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(&a - &b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(a - &b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(&a - b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));

        // Test for RHS value type
        {
            let mut c = Vec3::ONE;
            c -= a;
            assert_eq!(c, Vec3::new(1.0, 0.0, -1.0));
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::ONE;
            c -= &a;
            assert_eq!(c, Vec3::new(1.0, 0.0, -1.0));
        }
    }

    #[test]
    fn multiply() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(
            Vec3::new(0.0, 1.0, 2.0) * Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(0.0, 4.0, 10.0)
        );
        assert_eq!(a * Vec3::new(3.0, 4.0, 5.0), Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&a * Vec3::new(3.0, 4.0, 5.0), Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&a * &b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(a * &b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&a * b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(a * b, Vec3::new(0.0, 4.0, 10.0));

        // Test for RHS value type
        {
            let mut c = Vec3::splat(2.0);
            c *= a;
            assert_eq!(c, 2.0 * a);
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::splat(2.0);
            c *= &a;
            assert_eq!(c, 2.0 * a);
        }
    }

    #[test]
    fn divide() {
        let a = Vec3::new(1.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(
            Vec3::new(1.0, 1.0, 2.0) / Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0)
        );
        assert_eq!(
            a / Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0)
        );
        assert_eq!(
            &a / Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0)
        );
        assert_eq!(&a / &b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));
        assert_eq!(a / &b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));
        assert_eq!(&a / b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));
        assert_eq!(a / b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));

        // Test for RHS value type
        {
            let mut c = Vec3::ONE;
            c /= a;
            assert_eq!(c, Vec3::new(1.0, 1.0, 0.5));
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::ONE;
            c /= &a;
            assert_eq!(c, Vec3::new(1.0, 1.0, 0.5));
        }
    }

    #[test]
    fn dot() {
        let a = Vec3::new(2.0, 3.0, 5.0);
        let b = Vec3::new(7.0, 11.0, 13.0);
        assert_eq!(Vec3::dot(&a, &b), 2.0 * 7.0 + 3.0 * 11.0 + 5.0 * 13.0);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3::cross(&a, &b), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn len() {
        let a = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(a.len(), ((3.0 * 3.0 + 2.0 * 2.0 + 1.0 * 1.0) as f32).sqrt());

        let b = Vec3::splat(0.0);
        assert_eq!(b.len(), 0.0);
    }

    #[test]
    fn normalize() {
        let a = Vec3::new(3.0, 2.0, 1.0);
        let len = a.len();
        assert!((a.normalize().len() - 1.0).abs() < 0.01);
        assert_eq!(a.normalize(), a / len);
    }

    #[test]
    fn with_component() {
        let a = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(a.with_x(4.0), Vec3::new(4.0, 2.0, 1.0));
        assert_eq!(a.with_y(4.0), Vec3::new(3.0, 4.0, 1.0));
        assert_eq!(a.with_z(4.0), Vec3::new(3.0, 2.0, 4.0));
    }

    #[test]
    fn min() {
        let tiny_x = Vec3::new(0.00001, 1000.0, 1000.0);
        let tiny_y = Vec3::new(1000.0, 0.00001, 1000.0);
        let tiny_z = Vec3::new(1000.0, 1000.0, 0.00001);
        assert_eq!(tiny_x.min(&tiny_y).min(&tiny_z), Vec3::splat(0.00001));
    }

    #[test]
    fn max() {
        let big_x = Vec3::new(1000.0, 0.00001, 0.00001);
        let big_y = Vec3::new(0.00001, 1000.0, 0.00001);
        let big_z = Vec3::new(0.00001, 0.00001, 1000.0);
        assert_eq!(big_x.max(&big_y).max(&big_z), Vec3::splat(1000.0));
    }
}
