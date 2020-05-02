pub mod linalg {
    use alga::general::{ComplexField, Field};
    use ndarray::{arr1, Array1};
    use num_traits;
    use num_traits::identities::Zero;
    use std::cmp;
    use std::fmt;
    use std::ops::{Index, IndexMut, Neg};

    fn check_array_len<T: Field, U: Field>(vec1: &Array1<T>, vec2: &Array1<U>) {
        if vec1.len() > 0 && vec2.len() > 0 && vec1.len() != vec2.len() {
            panic!("Cannot operate on vectors with different non-zero sizes.");
        }
    }

    macro_rules! finite_space {
        ($cls_name:ident, $ndim:expr) => {
        pub struct $cls_name<T: Field> {
            elements: Array1<T>,
        }

        impl<T: Field + fmt::Display> fmt::Display for $cls_name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.elements)
            }
        }

        impl<T: Field> $cls_name<T> {
            pub fn new(data: [T; $ndim]) -> $cls_name<T> {
                Self::from_ndarray(arr1(&data))
            }

            pub fn from_ndarray(data: Array1<T>) -> $cls_name<T> {
                $cls_name { elements: data }
            }

            pub fn len(&self) -> usize {
                self.elements.len()
            }
        }

        impl<T: Field + Clone> Clone for $cls_name<T> {
            fn clone(&self) -> Self {
                $cls_name::from_ndarray(self.elements.clone())
            }
        }

        impl<T: Field> Neg for $cls_name<T> {
            type Output = $cls_name<T>;

            fn neg(self) -> Self::Output {
                return $cls_name::from_ndarray(-self.elements);
            }
        }

        impl<T: Field> Neg for &$cls_name<T> {
            type Output = $cls_name<T>;

            fn neg(self) -> Self::Output {
                return $cls_name::from_ndarray(-self.elements.clone());
            }
        }

        macro_rules! unary_ops {
            ($trait_name:ident, $func_name:ident, $op: tt) => {
                impl<U: Field, T: Field + std::ops::$trait_name<U>> std::ops::$trait_name<$cls_name<U>>
                    for $cls_name<T>
                {
                    fn $func_name(&mut self, other: $cls_name<U>) {
                        check_array_len(&self.elements, &other.elements);
                        let max_len = cmp::max(self.len(), other.len());
                        self.elements = Array1::zeros(max_len);
                        for i in 0..max_len {
                            self.elements[i] $op other.elements[i].clone();
                        }
                    }
                }

                impl<U: Field, T: Field + std::ops::$trait_name<U>> std::ops::$trait_name<U>
                    for $cls_name<T>
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
                    std::ops::$trait_name<$cls_name<U>> for $cls_name<T>
                {
                    type Output = $cls_name<Res>;
                    fn $func_name(self, rhs: $cls_name<U>) -> Self::Output {
                        check_array_len(&self.elements, &rhs.elements);

                        let mut res: Array1<Res> = Array1::zeros(self.len());
                        for (i, ele) in self.elements.iter().enumerate() {
                            res[i] = ele.clone() $op rhs.elements[i].clone();
                        }
                        $cls_name::from_ndarray(res)
                    }
                }

                impl<Res: Field, U: Field, T: Field + std::ops::$trait_name<U, Output = Res>>
                    std::ops::$trait_name<&$cls_name<U>> for &$cls_name<T>
                {
                    type Output = $cls_name<Res>;
                    fn $func_name(self, rhs: &$cls_name<U>) -> Self::Output {
                        check_array_len(&self.elements, &rhs.elements);

                        let mut res: Array1<Res> = Array1::zeros(self.len());
                        for (i, ele) in self.elements.iter().enumerate() {
                            res[i] = ele.clone() $op rhs.elements[i].clone();
                        }
                        $cls_name::from_ndarray(res)
                    }
                }

                impl<Res: Field, U: Field, T: Field + std::ops::$trait_name<U, Output = Res>>
                    std::ops::$trait_name<U> for $cls_name<T>
                {
                    type Output = $cls_name<Res>;

                    fn $func_name(self, rhs: U) -> Self::Output {
                        let mut res: Array1<Res> = Array1::zeros(self.len());
                        for (i, ele) in self.elements.iter().enumerate() {
                            res[i] = ele.clone() $op rhs.clone();
                        }
                        $cls_name::from_ndarray(res)
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

        impl<T: Field> Zero for $cls_name<T> {
            fn zero() -> Self {
                Self::from_ndarray(Array1::zeros($ndim))
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

        impl<T: Field + Zero + cmp::PartialEq> cmp::PartialEq for $cls_name<T> {
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

        impl<T: Field> alga::general::AbstractMagma<alga::general::Additive> for $cls_name<T> {
            fn operate(&self, _right: &Self) -> Self {
                unimplemented!()
            }
        }
        impl<T: Field> alga::general::TwoSidedInverse<alga::general::Additive> for $cls_name<T> {
            fn two_sided_inverse(&self) -> Self {
                -self
            }
        }
        impl<T: Field> alga::general::AbstractSemigroup<alga::general::Additive> for $cls_name<T> {}
        impl<T: Field> alga::general::AbstractQuasigroup<alga::general::Additive> for $cls_name<T> {}
        impl<T: Field> alga::general::Identity<alga::general::Additive> for $cls_name<T> {
            fn identity() -> Self {
                Self::zero()
            }
        }
        impl<T: Field> alga::general::AbstractLoop<alga::general::Additive> for $cls_name<T> {}
        impl<T: Field> alga::general::AbstractMonoid<alga::general::Additive> for $cls_name<T> {}
        impl<T: Field> alga::general::AbstractGroup<alga::general::Additive> for $cls_name<T> {}

        impl<T: Field> alga::general::AbstractGroupAbelian<alga::general::Additive> for $cls_name<T> {}

        impl<T: Field> alga::general::AbstractModule for $cls_name<T> {
            type AbstractRing = T;

            fn multiply_by(&self, r: Self::AbstractRing) -> Self {
                self.clone() * r
            }
        }

        impl<T: Field> alga::general::Module for $cls_name<T> {
            type Ring = T;
        }

        impl<T: ComplexField> alga::linear::VectorSpace for $cls_name<T> {
            type Field = T;
        }

        impl<T: ComplexField> alga::linear::NormedSpace for $cls_name<T> {
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

            fn try_normalize(&self, _eps: Self::RealField) -> Option<Self> {
                unimplemented!()
            }

            fn try_normalize_mut(&mut self, _eps: Self::RealField) -> Option<Self::RealField> {
                unimplemented!()
            }
        }

        impl<T: ComplexField> alga::linear::InnerSpace for $cls_name<T> {
            fn inner_product(&self, other: &Self) -> Self::ComplexField {
                let conjugate_vals = $cls_name::from_ndarray(
                    other
                        .elements
                        .map(|ele: &Self::ComplexField| -> Self::ComplexField { ele.conjugate() }),
                );
                (self * &conjugate_vals).elements.sum()
            }
        }

        impl<T: Field> Index<usize> for $cls_name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.elements[index]
            }
        }

        impl<T: Field> IndexMut<usize> for $cls_name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.elements[index]
            }
        }

        impl<T: ComplexField> alga::linear::FiniteDimVectorSpace for $cls_name<T> {
            fn dimension() -> usize {
                $ndim
            }

            fn canonical_basis_element(i: usize) -> Self {
                if i >= Self::dimension() {
                    panic!("Cannot find canonical basis at index {}", i);
                }
                let mut data: [T; $ndim] = [T::zero(); $ndim];
                data[i] = T::one();
                Self::new(data)
            }

            fn dot(&self, other: &Self) -> Self::Field {
                let conjugate_vals = $cls_name::from_ndarray(
                    other
                        .elements
                        .map(|ele: &Self::Field| -> Self::Field { ele.conjugate() }),
                );
                (self * &conjugate_vals).elements.sum()
            }

            unsafe fn component_unchecked(&self, i: usize) -> &Self::Field {
                &self.elements[i]
            }

            unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut Self::Field {
                &mut self.elements[i]
            }
        }

        impl<T: ComplexField> alga::linear::FiniteDimInnerSpace for $cls_name<T> {
            fn orthonormalize(_vs: &mut [Self]) -> usize {
                unimplemented!()
            }

            fn orthonormal_subspace_basis<F: FnMut(&Self) -> bool>(_vs: &[Self], _f: F) {
                unimplemented!()
            }
        }
        };
    }

    finite_space!(Vec0, 0);
    finite_space!(Vec1, 1);
    finite_space!(Vec2, 2);
    finite_space!(Vec3, 3);
}
