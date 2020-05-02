pub mod custom_type {
    use num_complex;
    pub type Real = f64;
    pub type Complex = num_complex::Complex<Real>;
}
