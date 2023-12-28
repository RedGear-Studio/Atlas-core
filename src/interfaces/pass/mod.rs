pub mod pass_error;

use crate::nodes::Program;

use self::pass_error::PassError;

pub trait Pass {
    fn apply(&mut self, program: &mut Program) -> Result<Program, Box<dyn PassError>>;   
}