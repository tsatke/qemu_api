mod accel;
mod drive;
mod log_item;
mod simple;

pub use accel::*;
pub use drive::*;
pub use log_item::*;
pub(crate) use simple::*;

pub trait QemuArgument {
    fn format(&self) -> Vec<String>;
}
