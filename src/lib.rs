use crate::args::{
    Accel, Bios, Drive, FreezeOnStartup, Fullscreen, Gdb, Help, LogItem, LogItems, NoReboot,
    Serial, Version,
};
use crate::chardev::QemuCharDevice;
use args::QemuArgument;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::Command;

pub mod args;
pub mod chardev;

pub trait QemuSystem {
    fn command() -> &'static str;
}

#[cfg(test)]
#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Generic;

#[cfg(test)]
impl QemuSystem for Generic {
    fn command() -> &'static str {
        "qemu-generic"
    }
}

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct X86_64;
impl QemuSystem for X86_64 {
    fn command() -> &'static str {
        "qemu-system-x86_64"
    }
}

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Aarch64;
impl QemuSystem for Aarch64 {
    fn command() -> &'static str {
        "qemu-system-aarch64"
    }
}

impl<S> From<Qemu<S>> for Command
where
    S: QemuSystem,
{
    fn from(value: Qemu<S>) -> Self {
        let mut cmd = Command::new(S::command());
        for arg in value.into_args() {
            cmd.arg(arg);
        }
        cmd
    }
}

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Qemu<S> {
    no_reboot: Option<NoReboot>,
    help: Option<Help>,
    version: Option<Version>,
    fullscreen: Option<Fullscreen>,
    accel: Option<Accel>,
    drives: Vec<Drive>,
    bios: Option<Bios>,
    serial: Option<Serial>,
    log_items: Option<LogItems>,
    freeze_on_startup: Option<FreezeOnStartup>,
    gdb: Option<Gdb>,
    _system: PhantomData<S>,
}

impl<S> Qemu<S>
where
    S: QemuSystem + Default,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<S> Qemu<S>
where
    S: QemuSystem,
{
    pub fn into_args(self) -> impl Iterator<Item = String> {
        self.args().into_iter()
    }

    fn args(self) -> Vec<String> {
        let mut args = Vec::new();

        // TODO: this could be done with a macro
        push_if_exists(&mut args, self.no_reboot);
        push_if_exists(&mut args, self.help);
        push_if_exists(&mut args, self.version);
        push_if_exists(&mut args, self.fullscreen);
        push_if_exists(&mut args, self.accel);
        push_many_if_exists(&mut args, self.drives);
        push_if_exists(&mut args, self.bios);
        push_if_exists(&mut args, self.serial);
        push_if_exists(&mut args, self.log_items);
        push_if_exists(&mut args, self.freeze_on_startup);
        push_if_exists(&mut args, self.gdb);

        args
    }

    pub fn no_reboot(&mut self) -> &mut Self {
        self.no_reboot = Some(NoReboot);
        self
    }

    pub fn help(&mut self) -> &mut Self {
        self.help = Some(Help);
        self
    }

    pub fn version(&mut self) -> &mut Self {
        self.version = Some(Version);
        self
    }

    pub fn fullscreen(&mut self) -> &mut Self {
        self.fullscreen = Some(Fullscreen);
        self
    }

    pub fn accel(&mut self, accel: Accel) -> &mut Self {
        self.accel = Some(accel);
        self
    }

    pub fn drive(&mut self, drive: Drive) -> &mut Self {
        self.drives.push(drive);
        self
    }

    pub fn bios(&mut self, bios: PathBuf) -> &mut Self {
        self.bios = Some(Bios(bios));
        self
    }

    pub fn serial(&mut self, serial_device: QemuCharDevice) -> &mut Self {
        self.serial = Some(Serial(serial_device));
        self
    }

    pub fn d<I>(&mut self, log_items: I) -> &mut Self
    where
        I: IntoIterator<Item = LogItem>,
    {
        self.log_items(log_items)
    }

    pub fn log_items<I>(&mut self, log_items: I) -> &mut Self
    where
        I: IntoIterator<Item = LogItem>,
    {
        let items = LogItems::from(log_items);
        self.log_items = Some(items);
        self
    }

    #[allow(non_snake_case)] // the qemu argument is called "-S", and "-s" is another argument
    pub fn S(&mut self) -> &mut Self {
        self.freeze_on_startup()
    }

    pub fn freeze_on_startup(&mut self) -> &mut Self {
        self.freeze_on_startup = Some(FreezeOnStartup);
        self
    }

    pub fn s(&mut self) -> &mut Self {
        self.gdb(&"tcp::1234")
    }

    pub fn gdb(&mut self, dev: &dyn AsRef<str>) -> &mut Self {
        let dev = dev.as_ref().to_string();
        self.gdb = Some(Gdb(dev));
        self
    }
}

fn push_if_exists<A>(vec: &mut Vec<String>, arg: Option<A>)
where
    A: QemuArgument,
{
    if let Some(arg) = arg {
        vec.extend(arg.format());
    }
}

fn push_many_if_exists<A>(vec: &mut Vec<String>, args: Vec<A>)
where
    A: QemuArgument,
{
    for arg in args {
        vec.extend(arg.format());
    }
}
