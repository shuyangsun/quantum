use alga::linear::{InnerSpace, NormedSpace};
use num_traits::*;
use quantum::vector::linalg::Vector;
use quantum::{Complex, Real};

fn main() {
    let vec_real: Vector<Real> = Vector::new(&[1.0, 2.0]);
    println!("vec_real = {} ", vec_real);

    let vec_complex: Vector<Complex> =
        Vector::new(&[Complex { re: 1.0, im: 2.0 }, Complex { re: 3.0, im: 4.0 }]);
    println!("vec_complex = {}", vec_complex);

    let mul_res: Vector<Complex> = &vec_real * &vec_complex;
    println!("mul_res = {}", mul_res);
    println!("mul_res is zero: {}", mul_res.is_zero());
    println!("{}", &mul_res * &mul_res);
    println!("mul_res norm_squared: {}", mul_res.norm_squared());
    println!(
        "mul_res inner_product(self, self): {}",
        mul_res.inner_product(&mul_res)
    );
    println!("mul_res norm: {}", mul_res.norm());
}
