pub mod binomial;
pub mod gaussian;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FamilyEnum {
    Binomial = 0,
    Gaussian = 1,
}
