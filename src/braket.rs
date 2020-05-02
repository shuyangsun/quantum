pub mod quantum_linalg {
    use crate::vector::linalg::Vector;
    use alga::general::Field;

    pub struct Ket<T: Field> {
        vec: Vector<T>,
    }

    pub struct Bra<T: Field> {
        vec: Vector<T>,
    }
}
