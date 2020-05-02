pub mod linalg {
    use alga::linear::VectorSpace;
    use ndarray::{arr1, Array1};
    use std::ops;

    pub struct OrderedBasis<V: VectorSpace> {
        basis: Array1<V>,
    }

    impl<V: VectorSpace> OrderedBasis<V> {
        pub fn new(data: &[V]) -> Self {
            Self { basis: arr1(data) }
        }

        pub fn from_ndarray(arr: Array1<V>) -> Self {
            Self { basis: arr }
        }
    }

    impl<V: VectorSpace> ops::Index<usize> for OrderedBasis<V> {
        type Output = V;

        fn index(&self, index: usize) -> &Self::Output {
            &self.basis[index]
        }
    }
}
