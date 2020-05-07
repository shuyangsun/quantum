use quantum::{OrderedOrthonormalBasis, SuperPosition};
extern crate nalgebra as na;
use alga::linear::InversibleSquareMatrix;
use na::{Matrix2, Vector2};

fn main() {
    let basis_1 = OrderedOrthonormalBasis::from(Matrix2::<f64>::identity());
    println!("{}", basis_1);
}
