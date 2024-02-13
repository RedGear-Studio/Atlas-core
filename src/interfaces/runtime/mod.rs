pub mod value;
pub mod visitor;

/// TODO
pub trait VisitorAlpha : Runtime {

}

pub trait Runtime {
    fn run();
}

use crate::prelude::error::Error;

pub trait RuntimeError: Error {}
