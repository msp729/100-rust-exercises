// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.
trait Power<I> {
    fn power(self, index: I) -> Self;
}

macro_rules! easy {
    ($b:ty:$h:ty) => {
        impl Power<$h> for $b {
            fn power(mut self, mut index: $h) -> Self {
                let mut r = 1;
                while index != 0 {
                    if index % 2 == 1 {
                        r *= self;
                    }
                    self *= self;
                    index /= 2;
                }
                r
            }
        }
    };
    ($($b:ty:$h:ty),+) => {
        $(easy!($b:$h);)+
    }
}

impl<A: Power<B>,B: Copy> Power<&B> for A {
    fn power(self, index: &B) -> A {self.power(*index)}
}

easy![u32:u16, u32:u32];

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
