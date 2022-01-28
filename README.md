# ベクトル演算ライブラリ
## 動機
昔、C++で作った数値演算ライブラリを発掘。
演算子のオーバーロードを使ったものだが、効率が結構悪かった記憶がある。
Rustでも演算子のオーバーロードができ、かつ、効率が良いらしいとのことなので、焼き直してみようと思い至った。
Rustだと、C++に比べて自動で型変換が行われないようなのでどれくらい使えるかみてみたい。

## 目標
短期目標としては、ライブラリとともにあった、ひまわりの種やボロノイ図作成・・・二次元図形ライブラリを実装する。

途中で、演算子のオーバーロードで実現可能なもの、無理なものもわかってくると思う。

## わかったこと
#### 演算子のオーバーロード

```rust
Vector<T> + Vector<T> -> Vector<T>
Vector<T> * T -> Vector<T>
T * Vector<T> -> Vector<T>
```
のように、二項演算で左項と結果が`Vector<T>`なら実装可能
ただし、下に示すように実装(impl)の対象がすべて異なる。特に3つ目のケースは、プリミティブ型の拡張になる。
更に3つ目のケースについてはジェネリックではなく、個別の型(例では`f64`)を必要分記述する。
```rust
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
impl Mul<Vector<f64>> for f64 {
    type Output = Vector<f64>;

    #[inline]
    fn mul(self, other: Self::Output) -> Self::Output {
        let w = self as f64;
        Vector {
            v: (w * other.v.0, w * other.v.1, w * other.v.2),
        }
    }
}
```
例では、
```rust
f64 * Vector<f64> -> Vector<f64>
```
と、結果の`Vector`も`f64`としているが
```rust
i32 * Vector<f64> -> Vector<f64>
```
のようにプリミティブ型とVector型の型を変えてしまうことも可能。
大量に定義が必要になるため、読みやすさのためには、マクロとして記述するのがおすすめ。

#### ジェネリック定義内での定数
Vector型で不足分の項目（例えば3次元を2次元として扱う場合の残りの次元）をゼロ初期化したい場合。
以下のような参照を使う
```rust
T::zero()
```

定義は
```rust
impl Zero for f64 {
    #[inline]
    fn zero() -> f64 {
        0.0
    }
}
```
の様に必要な型分だけ記述すれば可能になる。こちらも、大量に定義が必要になるのでマクロ化がおすすめ。


```rust
Vector<f32> + Vector<f64> -> Vector<f32>
```
のように型が`T`が左右項で異なる場合は、実装方法が不明か不可能
だが、先の演算子オーバーロードの例を見ると、
2項演算のすべての型の組み合わせについて定義して、記述すれば可能かもしれない。
Vector型の方は、f32とf64で良いと思うが、プリミティブ側が結構な数があり、結構な組み合わせ数となる。
実装はマクロを使えば、比較的簡単にできる。あとはどこまでするか・・・かな。


## 現状
ようやく先が見てきたかな。
