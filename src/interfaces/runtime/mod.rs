pub mod value;
pub mod visitor;
pub mod runtime_errors;

use self::value::Value;
/// TODO


pub trait VisitorAlpha : Runtime {

}

pub trait Runtime {
    fn run();
}