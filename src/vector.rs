use std::fmt;
use std::ops::{Add, Mul, Sub};

// T::zero()がほしいだけで、外部のcrateを使用。後で調べて、ゼロ化のところだけ取り込む！でも、すごく複雑！
use num::Num;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T> {
    v: (T, T, T),
}

impl<T> Vector<T>
where
    T: Add<Output = T> + Copy + Num + Mul<Output = T>,
{
    // pub fn zero() -> Vector<T> {
    //     Vector{ v: (T::zero(), 0.0, 0.0)}
    // }

    /// 生成子
    pub fn new(w: Vec<T>) -> Self {
        Self {
            v: (w[0], w[1], T::zero()),
        }
    }

    /// 生成子
    pub fn new2(x: T, y: T) -> Self {
        Self {
            v: (x, y, T::zero()),
        }
    }

    /// 要素参照
    pub fn get(&self) -> Vec<T> {
        vec![self.v.0, self.v.1, self.v.2]
    }

    /// スカラー積
    #[allow(non_snake_case)]
    pub fn aV(self, a: T) -> Self {
        Self::new2(self.v.0 * a, self.v.1 * a)
    }

    /// 内積
    pub fn dot(self, other: Self) -> T {
        self.v.0 * other.v.0 + self.v.1 * other.v.1
    }
}

/// format拡張
impl<T> fmt::Display for Vector<T>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.v.0, self.v.1)
    }
}

/// 加算拡張
impl<T> Add for Vector<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut w = self.v;
        w.0 = w.0 + other.v.0;
        w.1 = w.1 + other.v.1;
        Self { v: w }
    }
}

/// 減算拡張
impl<T> Sub for Vector<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut w = self.v;
        w.0 = w.0 - other.v.0;
        w.1 = w.1 - other.v.1;
        Self { v: w }
    }
}

// 式:`Vector<T> * T` の`*`は定義可能。
// 演算子の型と演算子の左項の型が不一致な場合の記述方法は
// 実現可能かどうかを含めて不明
/// スカラー積( V * a限定)拡張
impl<T> Mul<T> for Vector<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        let mut w = self.v;
        w.0 = w.0 * other;
        w.1 = w.1 * other;

        Self { v: w }
    }
}

/// スカラー積( V * a限定)拡張
impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Copy + Mul<Output = T> + Num + Sub<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut w = self;
        // w.v.0 = self.v.1 * other.v.2 - self.v.2 * other.v.1;
        // w.v.1 = self.v.2 * other.v.0 - self.v.0 * other.v.2;
        // w.v.2 = self.v.0 * other.v.1 - self.v.1 * other.v.0;
        // 現状、X-Y平面の2次元限定なので外積の結果はZ方向以外は強制的にゼロのする
        w.v.0 = T::zero();
        w.v.1 = T::zero();
        w.v.2 = self.v.0 * other.v.1 - self.v.1 * other.v.0;

        w
    }
}

/// 生成子
#[test]
fn test_new() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;

    let p2_0 = 2.4;
    let p2_1 = 3.9;

    let p1 = Vector::new(vec![p1_0, p1_1]);

    assert_eq!(p1.get()[0], p1_0);
    assert_eq!(p1.get()[1], p1_1);
    assert_eq!(p1.get()[2], 0.0);

    let p2 = Vector::new2(p2_0, p2_1);

    assert_eq!(p2.get()[0], p2_0);
    assert_eq!(p2.get()[1], p2_1);
    assert_eq!(p2.get()[2], 0.0);
}

/// 加算
#[test]
fn test_add() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;
    let p2_0 = 2.4;
    let p2_1 = 3.9;

    let p1 = Vector::new(vec![p1_0, p1_1]);
    let p2 = Vector::new2(p2_0, p2_1);

    assert_eq!((p1 + p2).get()[0], p1_0 + p2_0);
    assert_eq!((p1 + p2).get()[1], p1_1 + p2_1);
}

/// 減算
#[test]
fn test_sub() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;
    let p2_0 = 2.4;
    let p2_1 = 3.9;

    let p1 = Vector::new(vec![p1_0, p1_1]);
    let p2 = Vector::new2(p2_0, p2_1);

    assert_eq!((p1 - p2).get()[0], p1_0 - p2_0);
    assert_eq!((p1 - p2).get()[1], p1_1 - p2_1);
}

/// スカラー積 (V x a限定)
#[test]
#[allow(non_snake_case)]
fn test_aA() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;
    let a = 1.5;
    let b = 1.1;

    let p1 = Vector::new(vec![p1_0, p1_1]);

    assert_eq!((p1 * a).get()[0], p1_0 * a);
    assert_eq!((p1 * a).get()[1], p1_1 * a);

    assert_eq!((p1 * a * b).get()[0], p1_0 * a * b);
    assert_eq!((p1 * a * b).get()[1], p1_1 * a * b);
}

/// 桁落ち
#[test]
fn test_eq() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;
    let p2_0 = 2.4;
    let p2_1 = 3.9;

    let p1 = Vector::new(vec![p1_0, p1_1]);
    let p2 = Vector::new2(p2_0, p2_1);
    let p3 = Vector::new(vec![1.0, 0.5 - 1.0E-16]);
    // ここで桁落ち誤差に入る模様
    let p4 = Vector::new(vec![1.0, 0.5 - 1.0E-17]);

    assert_eq!(p1, p1);
    assert_ne!(p1, p2);
    assert_ne!(p1, p3);
    assert_eq!(p1, p4);
}

/// 内積
#[test]
fn test_dot() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;
    let p2_0 = 2.4;
    let p2_1 = 3.9;

    let p1 = Vector::new(vec![p1_0, p1_1]);
    let p2 = Vector::new2(p2_0, p2_1);

    let p3 = p1.dot(p2);
    let p4 = p2.dot(p1);

    assert_eq!(p3, p4);
    assert_eq!(p3, p1.get()[0] * p2.get()[0] + p1.get()[1] * p2.get()[1]);
}

/// 外積
#[test]
fn test_cross_product() {
    let p1_0 = 1.0;
    let p1_1 = 0.5;
    let p2_0 = 2.4;
    let p2_1 = 3.9;

    let p1 = Vector::new(vec![p1_0, p1_1]);
    let p2 = Vector::new2(p2_0, p2_1);

    let p3 = p1 * p2;

    assert_eq!(p3.get()[0], 0.0);
    assert_eq!(p3.get()[1], 0.0);

    let p4 = p2 * p1;

    assert_eq!(p4.get()[0], 0.0);
    assert_eq!(p4.get()[1], 0.0);

    // assert_ne!(p3.v()[2], 0.0);
    assert_eq!(
        p3.get()[2],
        p1.get()[0] * p2.get()[1] - p1.get()[1] * p2.get()[0]
    );
    assert_eq!(p3.get()[2] + p4.get()[2], 0.0);
}
