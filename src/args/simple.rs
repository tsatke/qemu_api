use crate::args::QemuArgument;
use std::path::PathBuf;

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct NoReboot;

impl QemuArgument for NoReboot {
    fn format(&self) -> Vec<String> {
        vec!["--no-reboot".to_string()]
    }
}

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Help;

impl QemuArgument for Help {
    fn format(&self) -> Vec<String> {
        vec!["--help".to_string()]
    }
}

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Version;

impl QemuArgument for Version {
    fn format(&self) -> Vec<String> {
        vec!["--version".to_string()]
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Bios(pub PathBuf);

impl QemuArgument for Bios {
    fn format(&self) -> Vec<String> {
        vec!["-bios".to_string(), self.0.display().to_string()]
    }
}

#[cfg(test)]
mod tests {
    use crate::Generic;
    use crate::Qemu;

    #[test]
    fn test_empty_args() {
        let qemu = Qemu::<Generic>::new();
        let args = qemu.args();
        assert!(args.is_empty());
    }

    #[test]
    fn test_no_reboot() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.no_reboot();

        let args = qemu.args();
        assert_eq!(&["--no-reboot"], args.as_slice());
    }

    #[test]
    fn test_help() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.help();

        let args = qemu.args();
        assert_eq!(&["--help"], args.as_slice());
    }

    #[test]
    fn test_version() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.version();

        let args = qemu.args();
        assert_eq!(&["--version"], args.as_slice());
    }
}
