use alga::general::ComplexField;
use alga::linear::{FiniteDimVectorSpace, InnerSpace, NormedSpace};
use num_traits::*;
use quantum::vector::linalg::{Vec2, Vec3};
use quantum::{Complex, Real};

fn main() {
    let vec_real: Vec2<Real> = Vec2::new([1.0, 2.0]);
    println!("vec_real = {} ", vec_real);

    let vec_complex: Vec2<Complex> =
        Vec2::new([Complex { re: 1.0, im: 2.0 }, Complex { re: 3.0, im: 4.0 }]);
    println!("vec_complex = {}", vec_complex);

    let std_real_0 = Vec2::new([1.0, 0.0]);
    let real_45 = Vec2::new([1.0, 1.0]);
    let real_angle = std_real_0.angle(&real_45);
    println!(
        "**** {}",
        std_real_0.inner_product(&real_45).real() * std_real_0.norm() * real_45.norm()
    );
    println!("Real angle: {}", real_angle);
    let complex_angle =
        Vec2::new([Complex { re: 1.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }]).angle(&Vec2::new(
            [Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
        ));
    println!("Complex angle: {}", complex_angle);

    let mul_res: Vec2<Complex> = &vec_real * &vec_complex;
    println!("mul_res = {}", mul_res);
    println!("mul_res is zero: {}", mul_res.is_zero());
    println!("mul_res norm_squared: {}", mul_res.norm_squared());
    println!(
        "mul_res inner_product(self, self): {}",
        mul_res.inner_product(&mul_res)
    );
    println!("Complex norm squared: {}", vec_complex.norm_squared());
    println!(
        "Complex inner product self: {}",
        vec_complex.inner_product(&vec_complex)
    );
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
