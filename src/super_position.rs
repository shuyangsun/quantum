pub mod super_position {
    use crate::utility::utility::Approximate;
    use crate::OrderedOrthonormalBasis;
    use alga::linear::{FiniteDimInnerSpace, FiniteDimVectorSpace, InversibleSquareMatrix};
    use num;
    use std::fmt;
    use std::ops::Mul;

    pub trait SuperPosMatrix<S: FiniteDimInnerSpace>:
        InversibleSquareMatrix + Mul<S, Output = S> + Mul<S::Field, Output = Self>
    {
    }
    pub trait SuperPosScalarVector<M: InversibleSquareMatrix>:
        FiniteDimInnerSpace + Mul<M::Field> + Clone
    {
    }

    pub struct SuperPosition<S, M>
    where
        M: SuperPosMatrix<S>,
        S: SuperPosScalarVector<M>,
    {
        scalars: S,
        basis: OrderedOrthonormalBasis<M>,
        vector_in_standard_basis: S,
    }

    impl<S, M> fmt::Display for SuperPosition<S, M>
    where
        M: SuperPosMatrix<S> + fmt::Display,
        S: SuperPosScalarVector<M> + fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "scalars: {}, basis: {}", self.scalars, self.basis)
        }
    }

    impl<S, M> SuperPosition<S, M>
    where
        M: SuperPosMatrix<S>,
        S: SuperPosScalarVector<M>,
    {
        fn from_scalars_and_basis(scalars: S, basis: OrderedOrthonormalBasis<M>) -> Self {
            debug_assert_eq!(S::dimension(), M::Vector::dimension());
            debug_assert!(scalars
                .norm_squared()
                .is_very_close(&num::one::<S::RealField>()));
            let standard_basis = OrderedOrthonormalBasis::<M>::standard();
            Self {
                scalars: scalars.clone(),
                basis: basis.clone(),
                vector_in_standard_basis: standard_basis.change_from_basis(scalars, basis),
            }
        }

        fn to_basis<S1, M1>(&self, new_basis: OrderedOrthonormalBasis<M1>) -> SuperPosition<S1, M1>
        where
            S1: SuperPosScalarVector<M1>,
            M1: SuperPosMatrix<S1> + Mul<S, Output = S1>,
        {
            let new_scalars =
                new_basis.change_from_standard_basis(self.vector_in_standard_basis.clone());
            SuperPosition::<S1, M1>::from_scalars_and_basis(new_scalars, new_basis)
        }
    }
}
