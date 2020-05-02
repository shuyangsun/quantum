pub mod linalg {
    use alga::general::{ComplexField, Field};
    use ndarray::{arr1, Array1};
    use num_traits;
    use std::cmp;
    use std::fmt;

    pub struct Vector<T: Field> {
        elements: Array1<T>,
    }

    impl<T: Field + fmt::Display> fmt::Display for Vector<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.elements)
        }
    }

    impl<T: Field> Vector<T> {
        pub fn new(data: &[T]) -> Vector<T> {
            Self::from_ndarray(arr1(data))
        }

        pub fn from_ndarray(data: Array1<T>) -> Vector<T> {
            Vector { elements: data }
        }

        pub fn len(&self) -> usize {
            self.elements.len()
        }
    }

    impl<T: Field + Clone> Clone for Vector<T> {
        fn clone(&self) -> Self {
            Vector::from_ndarray(self.elements.clone())
        }
    }

    impl<T: Field> std::ops::Neg for Vector<T> {
        type Output = Vector<T>;

        fn neg(self) -> Self::Output {
            return Vector::from_ndarray(-self.elements);
        }
    }

    macro_rules! unary_ops {
        ($trait_name:ident, $func_name:ident, $op: tt) => {
            impl<U: Field, T: Field + std::ops::$trait_name<U>> std::ops::$trait_name<Vector<U>>
                for Vector<T>
            {
                fn $func_name(&mut self, other: Vector<U>) {
                    check_vector_len(&self, &other);
                    let max_len = cmp::max(self.len(), other.len());
                    self.elements = Array1::zeros(max_len);
                    for i in 0..max_len {
                        self.elements[i] $op other.elements[i].clone();
                    }
                }
            }

            impl<U: Field, T: Field + std::ops::$trait_name<U>> std::ops::$trait_name<U>
                for Vector<T>
            {
                fn $func_name(&mut self, other: U) {
                    for ele in self.elements.iter_mut() {
                        *ele $op other.clone();
                    }
                }
            }
        };
    }

    macro_rules! binary_ops {
        ($trait_name:ident, $func_name:ident, $op: tt) => {
            impl<Res: Field, U: Field, T: Field + std::ops::$trait_name<U, Output = Res>>
                std::ops::$trait_name<Vector<U>> for Vector<T>
            {
                type Output = Vector<Res>;
                fn $func_name(self, rhs: Vector<U>) -> Self::Output {
                    check_vector_len(&self, &rhs);

                    let mut res: Array1<Res> = Array1::zeros(self.len());
                    for (i, ele) in self.elements.iter().enumerate() {
                        res[i] = ele.clone() $op rhs.elements[i].clone();
                    }
                    Vector::from_ndarray(res)
                }
            }

            impl<Res: Field, U: Field, T: Field + std::ops::$trait_name<U, Output = Res>>
                std::ops::$trait_name<&Vector<U>> for &Vector<T>
            {
                type Output = Vector<Res>;
                fn $func_name(self, rhs: &Vector<U>) -> Self::Output {
                    check_vector_len(self, rhs);

                    let mut res: Array1<Res> = Array1::zeros(self.len());
                    for (i, ele) in self.elements.iter().enumerate() {
                        res[i] = ele.clone() $op rhs.elements[i].clone();
                    }
                    Vector::from_ndarray(res)
                }
            }

            impl<Res: Field, U: Field, T: Field + std::ops::$trait_name<U, Output = Res>>
                std::ops::$trait_name<U> for Vector<T>
            {
                type Output = Vector<Res>;

                fn $func_name(self, rhs: U) -> Self::Output {
                    let mut res: Array1<Res> = Array1::zeros(self.len());
                    for (i, ele) in self.elements.iter().enumerate() {
                        res[i] = ele.clone() $op rhs.clone();
                    }
                    Vector::from_ndarray(res)
                }
            }
        };
    }

    binary_ops!(Add, add, +);
    binary_ops!(Sub, sub, -);
    binary_ops!(Mul, mul, *);
    unary_ops!(AddAssign, add_assign, +=);
    unary_ops!(SubAssign, sub_assign, -=);
    unary_ops!(MulAssign, mul_assign, *=);

    impl<T: Field> num_traits::identities::Zero for Vector<T> {
        fn zero() -> Self {
            Vector {
                elements: Array1::zeros(0),
            }
        }

        fn is_zero(&self) -> bool {
            if self.len() <= 0 {
                true
            } else {
                for ele in self.elements.iter() {
                    if !ele.is_zero() {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    impl<T: Field + num_traits::identities::Zero + cmp::PartialEq> cmp::PartialEq for Vector<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.len() != other.len() {
                for ele in self.elements.iter() {
                    if !ele.is_zero() {
                        return false;
                    }
                }
                for ele in other.elements.iter() {
                    if !ele.is_zero() {
                        return false;
                    }
                }
                return true;
            }

            for (i, ele) in self.elements.iter().enumerate() {
                if *ele != other.elements[i] {
                    return false;
                }
            }
            return true;
        }
    }

    impl<T: Field> alga::general::AbstractMagma<alga::general::Additive> for Vector<T> {
        fn operate(&self, right: &Self) -> Self {
            unimplemented!()
        }
    }
    impl<T: Field> alga::general::TwoSidedInverse<alga::general::Additive> for Vector<T> {
        fn two_sided_inverse(&self) -> Self {
            unimplemented!()
        }
    }
    impl<T: Field> alga::general::AbstractSemigroup<alga::general::Additive> for Vector<T> {}
    impl<T: Field> alga::general::AbstractQuasigroup<alga::general::Additive> for Vector<T> {}
    impl<T: Field> alga::general::Identity<alga::general::Additive> for Vector<T> {
        fn identity() -> Self {
            unimplemented!()
        }
    }
    impl<T: Field> alga::general::AbstractLoop<alga::general::Additive> for Vector<T> {}
    impl<T: Field> alga::general::AbstractMonoid<alga::general::Additive> for Vector<T> {}
    impl<T: Field> alga::general::AbstractGroup<alga::general::Additive> for Vector<T> {}

    impl<T: Field> alga::general::AbstractGroupAbelian<alga::general::Additive> for Vector<T> {}

    impl<T: Field> alga::general::AbstractModule for Vector<T> {
        type AbstractRing = T;

        fn multiply_by(&self, r: Self::AbstractRing) -> Self {
            self.clone() * r
        }
    }

    impl<T: Field> alga::general::Module for Vector<T> {
        type Ring = T;
    }

    impl<T: ComplexField> alga::linear::VectorSpace for Vector<T> {
        type Field = T;
    }

    impl<T: ComplexField> alga::linear::NormedSpace for Vector<T> {
        type RealField = T::RealField;
        type ComplexField = T;

        fn norm_squared(&self) -> Self::RealField {
            let abs_vals_squared = self.elements.map(|ele: &T| -> Self::RealField {
                let abs_val = ele.abs();
                abs_val.clone() * abs_val
            });
            abs_vals_squared.sum()
        }

        fn norm(&self) -> Self::RealField {
            self.norm_squared().sqrt()
        }

        fn normalize(&self) -> Self {
            unimplemented!()
        }

        fn normalize_mut(&mut self) -> Self::RealField {
            unimplemented!()
        }

        fn try_normalize(&self, eps: Self::RealField) -> Option<Self> {
            unimplemented!()
        }

        fn try_normalize_mut(&mut self, eps: Self::RealField) -> Option<Self::RealField> {
            unimplemented!()
        }
    }

    impl<T: ComplexField> alga::linear::InnerSpace for Vector<T> {
        fn inner_product(&self, other: &Self) -> Self::ComplexField {
            let conjugate_vals = Vector::from_ndarray(
                other
                    .elements
                    .map(|ele: &Self::ComplexField| -> Self::ComplexField { ele.conjugate() }),
            );
            (self * &conjugate_vals).elements.sum()
        }
    }

    fn check_vector_len<T: Field, U: Field>(vec1: &Vector<T>, vec2: &Vector<U>) {
        if vec1.len() > 0 && vec2.len() > 0 && vec1.len() != vec2.len() {
            panic!("Cannot operate on vectors with different non-zero sizes.");
        }
    }
}
