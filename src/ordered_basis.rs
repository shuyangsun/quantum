pub mod linalg {
    use alga::linear::{FiniteDimVectorSpace, InversibleSquareMatrix};
    use std::cmp::{Eq, PartialEq};
    use std::fmt;
    use std::ops::Mul;

    #[derive(Clone, Debug)]
    pub struct OrderedOrthonormalBasis<M>
    where
        M: InversibleSquareMatrix,
    {
        matrix: M,
        inverse: M,
    }

    impl<M> fmt::Display for OrderedOrthonormalBasis<M>
    where
        M: InversibleSquareMatrix + fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.matrix)
        }
    }

    impl<M> OrderedOrthonormalBasis<M>
    where
        M: InversibleSquareMatrix,
    {
        pub fn from(matrix: M) -> Self {
            Self {
                matrix: matrix.clone(),
                inverse: matrix.transpose(),
            }
        }

        pub fn standard() -> Self {
            Self::from(M::identity())
        }

        pub fn dimension() -> usize {
            M::Vector::dimension()
        }

        pub fn get(&self, index: usize) -> M::Vector {
            self.matrix.column(index)
        }

        pub fn change_from_basis<V1, V2, V3, M1>(
            &self,
            old_vector: V1,
            old_basis: OrderedOrthonormalBasis<M1>,
        ) -> V3
        where
            V1: FiniteDimVectorSpace,
            V2: FiniteDimVectorSpace,
            V3: FiniteDimVectorSpace,
            M: InversibleSquareMatrix + Mul<V2, Output = V3>,
            M1: InversibleSquareMatrix + Mul<V1, Output = V2>,
        {
            self.inverse.clone() * (old_basis.matrix * old_vector)
        }

        pub fn change_from_standard_basis<V1, V2>(&self, vector_in_standard_basis: V1) -> V2
        where
            V1: FiniteDimVectorSpace,
            M: InversibleSquareMatrix + Mul<V1, Output = V2>,
        {
            self.inverse.clone() * vector_in_standard_basis
        }
    }

    impl<M> PartialEq for OrderedOrthonormalBasis<M>
    where
        M: InversibleSquareMatrix,
    {
        fn eq(&self, other: &Self) -> bool {
            self.matrix == other.matrix
        }
    }

    impl<M> Eq for OrderedOrthonormalBasis<M> where M: InversibleSquareMatrix {}
}
