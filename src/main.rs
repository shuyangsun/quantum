use alga::linear::{FiniteDimVectorSpace, InnerSpace, NormedSpace};
use num_traits::*;
use quantum::vector::linalg::{Vec2, Vec3};
use quantum::{Complex, Real};

fn main() {
    // struct A<T: ComplexField> {
    //     x: T,
    // }
    //
    // impl<T: ComplexField> A<T> {
    //     fn hey() {
    //         T::zero()
    //     }
    // }
    // let mut asdf: [i32; 5];
    // for ele in asdf.iter_mut() {
    //     *ele = 0;
    // }
    let vec_real: Vec2<Real> = Vec2::new([1.0, 2.0]);
    println!("vec_real = {} ", vec_real);

    let vec_complex: Vec2<Complex> =
        Vec2::new([Complex { re: 1.0, im: 2.0 }, Complex { re: 3.0, im: 4.0 }]);
    println!("vec_complex = {}", vec_complex);

    let mul_res: Vec2<Complex> = &vec_real * &vec_complex;
    println!("mul_res = {}", mul_res);
    println!("mul_res is zero: {}", mul_res.is_zero());
    println!("mul_res norm_squared: {}", mul_res.norm_squared());
    println!(
        "mul_res inner_product(self, self): {}",
        mul_res.inner_product(&mul_res)
    );
    println!("Complex norm squared: {}", vec_complex.norm_squared());
    println!("Complex dot self: {}", vec_complex.dot(&vec_complex));
    println!("mul_res norm: {}", mul_res.norm());
    println!(
        "Canonical basis of R^2: [{}, {}]",
        Vec2::<Real>::canonical_basis_element(0),
        Vec2::<Real>::canonical_basis_element(1)
    );
    println!(
        "Canonical basis of C^3: [{}, {}, {}]",
        Vec3::<Complex>::canonical_basis_element(0),
        Vec3::<Complex>::canonical_basis_element(1),
        Vec3::<Complex>::canonical_basis_element(2)
    );
}
