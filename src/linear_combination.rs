pub mod super_position {
    use alga::general::{ComplexField, Field};
    use alga::linear::VectorSpace;
    use ndarray::Array1;

    pub struct SuperPosition<S: ComplexField, V: VectorSpace> {
        scalars: Array1<S>,
        ordered_basis: Array1<V>,
    }

    impl<S: ComplexField, V: VectorSpace> SuperPosition<S, V> {
        pub fn new(scalars: Array1<S>, ordered_basis: Array1<V>) -> Self {
            Self {
                scalars,
                ordered_basis,
            }
        }
    }
}
