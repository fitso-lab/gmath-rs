use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Sub};

use crate::sqrt::Sqrt;
use crate::zero::Zero;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<T> {
    v: (T, T, T),
}

impl<T> Vector<T>
where
    T: Zero,
{
    pub fn new(v: (T, T, T)) -> Self {
        Self { v }
    }

    pub fn new2(v: (T, T)) -> Self {
        Self {
            v: (v.0, v.1, T::zero()),
        }
    }

    pub fn zero() -> Self {
        Self::new((T::zero(), T::zero(), T::zero()))
    }
}

impl<T> Vector<T>
where
    T: Copy,
{
    /// 要素参照
    pub fn v(&self) -> (T, T, T) {
        (self.v.0, self.v.1, self.v.2)
    }
}

/// V + V -> V
impl<T> Add<Vector<T>> for Vector<T>
where
    T: Add<Output = T>,
    T: Zero,
{
    type Output = Vector<T>;

    #[inline]
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::new((self.v.0 + rhs.v.0, self.v.1 + rhs.v.1, self.v.2 + rhs.v.2))
    }
}

/// V - V -> V
impl<T> Sub<Vector<T>> for Vector<T>
where
    T: Sub<Output = T>,
    T: Zero,
{
    type Output = Vector<T>;

    #[inline]
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::new((self.v.0 - rhs.v.0, self.v.1 - rhs.v.1, self.v.2 - rhs.v.2))
    }
}

/// V * V -> V
/// 外積拡張
impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Clone,
    T: Mul<Output = T>,
    T: Sub<Output = T>,
    T: Zero,
{
    type Output = Vector<T>;

    #[inline]
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self::new((
            self.v.1 * rhs.v.2 - self.v.2 * rhs.v.1,
            self.v.2 * rhs.v.0 - self.v.0 * rhs.v.2,
            self.v.0 * rhs.v.1 - self.v.1 * rhs.v.0,
        ))
    }
}

/// V * a -> V
impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T>,
    T: Clone,
    T: Zero,
{
    type Output = Vector<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new((
            self.v.0 * rhs.clone(),
            self.v.1 * rhs.clone(),
            self.v.2 * rhs,
        ))
    }
}

/// a * V -> V
macro_rules! vector_aV_impl {
    ($tgt:ty, $out:ty) => {
        impl Mul<Vector<$out>> for $tgt {
            type Output = Vector<$out>;

            #[inline]
            fn mul(self, other: Self::Output) -> Self::Output {
                let w = self as $out;
                Vector {
                    v: (w * other.v.0, w * other.v.1, w * other.v.2),
                }
            }
        }
    };
}

vector_aV_impl!(f64, f64);
vector_aV_impl!(f32, f32);
vector_aV_impl!(i32, f64);

// impl Mul<Vector<f64>> for f64 {
//     type Output = Vector<f64>;

//     #[inline]
//     fn mul(self, other: Self::Output) -> Self::Output {
//         Vector {
//             v: (self * other.v.0, self * other.v.1, self * other.v.2),
//         }
//     }
// }

// impl Mul<Vector<f32>> for f32 {
//     type Output = Vector<f32>;

//     #[inline]
//     fn mul(self, other: Self::Output) -> Self::Output {
//         Vector {
//             v: (self * other.v.0, self * other.v.1, self * other.v.2),
//         }
//     }
// }

// impl Mul<Vector<f32>> for i32 {
//     type Output = Vector<f32>;

//     #[inline]
//     fn mul(self, other: Self::Output) -> Self::Output {
//         let w = self as f32;
//         Vector {
//             v: (w * other.v.0, w * other.v.1, w * other.v.2),
//         }
//     }
// }

impl<T> Display for Vector<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.v.0, self.v.1, self.v.2)
    }
}

impl<T> Vector<T>
where
    T: Add<Output = T>,
    T: Copy,
    T: Mul<Output = T>,
    T: PartialEq,
    T: Sqrt,
    T: Zero,
{
    /// 内積
    pub fn dot(self, other: Self) -> T {
        self.v.0 * other.v.0 + self.v.1 * other.v.1
    }

    /// 長さ
    pub fn len(self) -> T {
        T::sqrt(self.dot(self))
    }

    /// ゼロ?
    pub fn is_zero(self) -> bool {
        self.len() == T::zero()
    }
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! assert_eq_vector_VpV {
    ($a:expr, $b:expr, $msg:expr) => {
        assert_eq!(($a + $b).v.0, $a.v.0 + $b.v.0, $msg);
        assert_eq!(($a + $b).v.1, $a.v.1 + $b.v.1, $msg);
        assert_eq!(($a + $b).v.2, $a.v.2 + $b.v.2, $msg);

        assert_eq!(($b + $a).v.0, $b.v.0 + $a.v.0, $msg);
        assert_eq!(($b + $a).v.1, $b.v.1 + $a.v.1, $msg);
        assert_eq!(($b + $a).v.2, $b.v.2 + $a.v.2, $msg);
    };
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! assert_eq_vector_VmV {
    ($a:expr, $b:expr, $msg:expr) => {
        assert_eq!(($a - $b).v.0, $a.v.0 - $b.v.0, $msg);
        assert_eq!(($a - $b).v.1, $a.v.1 - $b.v.1, $msg);
        assert_eq!(($a - $b).v.2, $a.v.2 - $b.v.2, $msg);

        assert_eq!(($b - $a).v.0, $b.v.0 - $a.v.0, $msg);
        assert_eq!(($b - $a).v.1, $b.v.1 - $a.v.1, $msg);
        assert_eq!(($b - $a).v.2, $b.v.2 - $a.v.2, $msg);
    };
}

#[cfg(test)]
macro_rules! assert_eq_vector_Va {
    ($a:expr, $b:expr, $msg:expr) => {
        assert_eq!(($a * $b).v.0, $a.v.0 * $b, $msg);
        assert_eq!(($a * $b).v.1, $a.v.1 * $b, $msg);
        assert_eq!(($a * $b).v.2, $a.v.2 * $b, $msg);

        assert_eq!(($b * $a).v.0, $b * $a.v.0, $msg);
        assert_eq!(($b * $a).v.1, $b * $a.v.1, $msg);
        assert_eq!(($b * $a).v.2, $b * $a.v.2, $msg);
    };
}

#[test]
fn test() {
    let a = 2.0f64;
    let a_1 = Vector::new((1.1, 1.2, 0.0));
    let a_2 = Vector::new((1.3, 1.4, 0.0));

    let b = 3.0;
    let b_1 = Vector {
        v: (2.1 as f32, 2.2, 0.0),
    };
    let _b_2 = Vector {
        v: (2.3 as f32, 2.4, 0.0),
    };

    let _c = 3;

    let _r_1 = a_1 * a;
    let _r_2 = a * a_1;
    let _r_3 = a_1 + a_2;

    assert_eq_vector_Va!(a_1, a, "V * a");

    // assert_eq_vector_Va!(a_1, c, "V * a");

    // println!("V[{}] * c[{}] = [{}]", a_1, c, a_1 * c);
    // println!("c[{}] * V[{}] = [{}]", c, a_1, c * a_1);

    assert_eq_vector_Va!(b_1, b, "V * a");

    // println!("a[{}] * V[{}] = [{}]", a, b_1, a * b_1);
}

/// 生成子
#[test]
fn test_new() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;

    let p2_0: f64 = 2.4;
    let p2_1: f64 = 3.9;

    let p1 = Vector::new((p1_0, p1_1, 0.0));

    assert_eq!(p1.v().0, p1_0);
    assert_eq!(p1.v().1, p1_1);
    assert_eq!(p1.v().2, 0.0);

    let p2 = Vector::new2((p2_0, p2_1));

    assert_eq!(p2.v().0, p2_0);
    assert_eq!(p2.v().1, p2_1);
    assert_eq!(p2.v().2, 0.0);
}

/// ゼロ
#[test]
fn test_zero() {
    let p1 = Vector::<f64>::zero();

    assert_eq!(p1.v().0, 0.0);
    assert_eq!(p1.v().1, 0.0);
    assert_eq!(p1.v().2, 0.0);
}

/// 加算
#[test]
fn test_add() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;
    let p2_0: f64 = 2.4;
    let p2_1: f64 = 3.9;

    let p1 = Vector::new((p1_0, p1_1, 0.0));
    let p2 = Vector::new2((p2_0, p2_1));

    assert_eq!((p1 + p2).v().0, p1_0 + p2_0);
    assert_eq!((p1 + p2).v().1, p1_1 + p2_1);
}

/// 減算
#[test]
fn test_sub() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;
    let p2_0: f64 = 2.4;
    let p2_1: f64 = 3.9;

    let p1 = Vector::new((p1_0, p1_1, 0.0));
    let p2 = Vector::new2((p2_0, p2_1));

    assert_eq!((p1 - p2).v.0, p1_0 - p2_0);
    assert_eq!((p1 - p2).v.1, p1_1 - p2_1);
}

/// スカラー積 (V x a)
#[test]
#[allow(non_snake_case)]
fn test_Va() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;

    let p1 = Vector::new((p1_0, p1_1, 0.0));

    let a = 1.5;
    let b = 1.1;

    assert_eq!((p1 * a).v().0, p1_0 * a);
    assert_eq!((p1 * a).v().1, p1_1 * a);

    assert_eq!((p1 * a * b).v.0, p1_0 * a * b);
    assert_eq!((p1 * a * b).v.1, p1_1 * a * b);
}

/// スカラー積 (a x V)
#[test]
#[allow(non_snake_case)]
fn test_aV() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;

    let p1 = Vector::new((p1_0, p1_1, 0.0));

    let a = 1.5;
    let b = 1.1;

    assert_eq!((a * p1).v().0, p1_0 * a);
    assert_eq!((a * p1).v().1, p1_1 * a);

    assert_eq!((a * b * p1).v.0, p1_0 * a * b);
    assert_eq!((a * b * p1).v.1, p1_1 * a * b);
}

/// 桁落ち
#[test]
fn test_eq() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;
    let p2_0: f64 = 2.4;
    let p2_1: f64 = 3.9;

    let p1 = Vector::new((p1_0, p1_1, 0.0));
    let p2 = Vector::new2((p2_0, p2_1));
    let p3 = Vector::new2((1.0, 0.5 - 1.0E-16));
    // ここで桁落ち誤差に入る模様
    let p4 = Vector::new2((1.0, 0.5 - 1.0E-17));

    assert_eq!(p1, p1);
    assert_ne!(p1, p2);
    assert_ne!(p1, p3);
    assert_eq!(p1, p4);
}

/// 内積
#[test]
fn test_dot() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;
    let p2_0: f64 = 2.4;
    let p2_1: f64 = 3.9;

    let p1 = Vector::new((p1_0, p1_1, 0.0));
    let p2 = Vector::new2((p2_0, p2_1));

    let p3 = p1.dot(p2);
    let p4 = p2.dot(p1);

    assert_eq!(p3, p4);
    assert_eq!(p3, p1.v.0 * p2.v.0 + p1.v().1 * p2.v().1);
}

/// 外積
#[test]
fn test_cross_product() {
    let p1_0: f64 = 1.0;
    let p1_1: f64 = 0.5;
    let p2_0: f64 = 2.4;
    let p2_1: f64 = 3.9;

    let p1 = Vector::new((p1_0, p1_1, 0.0));
    let p2 = Vector::new2((p2_0, p2_1));

    let p3 = p1 * p2;

    assert_eq!(p3.v.0, 0.0);
    assert_eq!(p3.v.1, 0.0);

    let p4 = p2 * p1;

    assert_eq!(p4.v.0, 0.0);
    assert_eq!(p4.v.1, 0.0);

    // assert_ne!(p3.v()[2], 0.0);
    assert_eq!(p3.v().2, p1.v().0 * p2.v().1 - p1.v().1 * p2.v().0);
    assert_eq!(p3.v().2 + p4.v().2, 0.0);
}
