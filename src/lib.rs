use crate::args::{Accel, Bios, Drive, Help, NoReboot, Version};
use args::QemuArgument;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::Command;

pub mod args;

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
    accel: Option<Accel>,
    drives: Vec<Drive>,
    bios: Option<Bios>,
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
        push_if_exists(&mut args, self.accel);
        push_many_if_exists(&mut args, self.drives);
        push_if_exists(&mut args, self.bios);

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
