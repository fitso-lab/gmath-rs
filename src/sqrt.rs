pub trait Sqrt {
    fn sqrt(self) -> Self;
}

#[allow(unused_macros)]
macro_rules! sqrt_impl {
    ($t:ty) => {
        impl Sqrt for $t {
            #[inline]
            fn sqrt(self) -> $t {
                <$t>::sqrt(self)
            }
        }
    };
}
sqrt_impl!(f32);
sqrt_impl!(f64);
