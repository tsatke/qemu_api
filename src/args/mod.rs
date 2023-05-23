mod accel;
mod drive;
mod simple;

pub use accel::*;
pub use drive::*;
pub(crate) use simple::*;

pub trait QemuArgument {
    fn format(&self) -> Vec<String>;
}
