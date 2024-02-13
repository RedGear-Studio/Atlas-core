use crate::nodes::Program;
use super::error::Error;

pub trait Pass<'p> {
    fn apply(program: &'p mut Program) -> Result<Program<'p>, Box<dyn PassError>>;   
}

pub trait PassError: Error {}
