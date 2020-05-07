pub mod utility {
    use alga::general::{ComplexField, Field, RealField};
    use alga::linear::InversibleSquareMatrix;
    use nalgebra::Matrix2;
    use num;
    use num_complex;

    pub type Real = f64;
    pub type Complex = num_complex::Complex<Real>;

    pub trait DefaultEpsilon<T>
    where
        T: RealField,
    {
        fn ep_7() -> T {
            let one = num::one::<T::RealField>();
            let two = one + one.clone();
            let half = one.unscale(two.clone());
            let fourth = half * half;
            let sixteenth = fourth * fourth;
            let two_fifty_sixth = sixteenth * sixteenth;
            let big_frac = two_fifty_sixth * two_fifty_sixth;
            big_frac.unscale(sixteenth * fourth * half) // 1.1920929e-7
        }
    }

    impl<T> DefaultEpsilon<T> for T where T: RealField {}

    pub trait RoundTo<P>
    where
        P: RealField,
    {
        fn round_to(&self, ep: P) -> Self;
        fn round_small(&self) -> Self;
    }

    pub trait Approximate<P>
    where
        P: RealField,
    {
        fn is_close(&self, other: &Self, ep: P) -> bool;
        fn is_very_close(&self, other: &Self) -> bool;
    }

    impl<T> RoundTo<T::RealField> for T
    where
        T: ComplexField,
    {
        fn round_to(&self, ep: T::RealField) -> Self {
            self.unscale(ep.clone()).round().scale(ep)
        }

        fn round_small(&self) -> Self {
            self.round_to(T::RealField::ep_7())
        }
    }

    impl<T> Approximate<T::RealField> for T
    where
        T: ComplexField,
    {
        fn is_close(&self, other: &Self, ep: T::RealField) -> bool {
            self.round_to(ep.clone()) == other.round_to(ep)
        }

        fn is_very_close(&self, other: &Self) -> bool {
            self.is_close(other, T::RealField::ep_7())
        }
    }
}
