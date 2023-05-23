use crate::args::QemuArgument;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Accel {
    pub accelerator: Accelerator,
    pub idg_passthru: IdgPassthru,
    pub kernel_irqchip: KernelIrqchip,
    pub split_wx: Option<SplitWx>,
    pub thread: Option<Thread>,
    pub kvm_shadow_mem: Option<usize>,
    pub tb_size: Option<usize>,
    pub dirty_ring_size: Option<usize>, // default is 0, but we probably shouldn't set it by default, since this is a kvm option
    pub notify_vmexit: Option<NotifyVmexit>,
}

impl QemuArgument for Accel {
    fn format(&self) -> Vec<String> {
        let mut res = format!(
            "accel={},idg-passthru={},kernel-irqchip={}",
            self.accelerator.as_str(),
            self.idg_passthru.as_str(),
            self.kernel_irqchip.as_str(),
        );
        if let Some(split_wx) = self.split_wx {
            res.push_str(&format!(",split-wx={}", split_wx.as_str()));
        }
        if let Some(thread) = self.thread {
            res.push_str(&format!(",thread={}", thread.as_str()));
        }
        if let Some(kvm_shadow_mem) = self.kvm_shadow_mem {
            res.push_str(&format!(",kvm-shadow-mem={}", kvm_shadow_mem));
        }
        if let Some(tb_size) = self.tb_size {
            res.push_str(&format!(",tb-size={}", tb_size));
        }
        if let Some(dirty_ring_size) = self.dirty_ring_size {
            res.push_str(&format!(",dirty-ring-size={}", dirty_ring_size));
        }
        if let Some(notify_vmexit) = self.notify_vmexit {
            res.push_str(&format!(",notify-vmexit={}", notify_vmexit.format()));
        }

        vec!["-accel".to_string(), res]
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Accelerator {
    Kvm,
    Xen,
    Hax,
    Hvf,
    Nvmm,
    Whpx,
    Tcg,
    #[default]
    Help,
}

impl Accelerator {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Kvm => "kvm",
            Self::Xen => "xen",
            Self::Hax => "hax",
            Self::Hvf => "hvf",
            Self::Nvmm => "nvmm",
            Self::Whpx => "whpx",
            Self::Tcg => "tcg",
            Self::Help => "help",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum IdgPassthru {
    On,
    #[default]
    Off,
}

impl IdgPassthru {
    fn as_str(&self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum KernelIrqchip {
    #[default]
    On,
    Off,
    Split,
}

impl KernelIrqchip {
    fn as_str(&self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Split => "split",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SplitWx {
    On,
    Off,
}

impl SplitWx {
    fn as_str(&self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Thread {
    Single,
    Multi,
}

impl Thread {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multi => "multi",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NotifyVmexit {
    Run,
    InternalError,
    DisableAndNotifyWindow(usize),
}

impl NotifyVmexit {
    fn format(&self) -> String {
        match self {
            Self::Run => "run".to_string(),
            Self::InternalError => "internal-error".to_string(),
            Self::DisableAndNotifyWindow(window) => {
                format!("disable,notify-window={}", window)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Generic;
    use crate::Qemu;

    #[test]
    fn test_accel_and_defaults() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &["-accel", "accel=kvm,idg-passthru=off,kernel-irqchip=on"],
            args.as_slice()
        );
    }

    #[test]
    fn test_idg_passthru() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            idg_passthru: IdgPassthru::On,
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &["-accel", "accel=kvm,idg-passthru=on,kernel-irqchip=on"],
            args.as_slice()
        );
    }

    #[test]
    fn test_no_kernel_irqchip() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            kernel_irqchip: KernelIrqchip::Off,
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &["-accel", "accel=kvm,idg-passthru=off,kernel-irqchip=off"],
            args.as_slice()
        );
    }

    #[test]
    fn test_split_wx_on() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            split_wx: Some(SplitWx::On),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,split-wx=on"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_split_wx_off() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            split_wx: Some(SplitWx::Off),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,split-wx=off"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_thread_single() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            thread: Some(Thread::Single),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,thread=single"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_thread_multi() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            thread: Some(Thread::Multi),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,thread=multi"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_kvm_shadow_mem() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            kvm_shadow_mem: Some(3735928559),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,kvm-shadow-mem=3735928559"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_tb_size() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            tb_size: Some(123),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,tb-size=123"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_dirty_ring_size() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            dirty_ring_size: Some(1337),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,dirty-ring-size=1337"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_notify_vmexit_run() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            notify_vmexit: Some(NotifyVmexit::Run),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,notify-vmexit=run"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_notify_vmexit_internal_error() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            notify_vmexit: Some(NotifyVmexit::InternalError),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,notify-vmexit=internal-error"
            ],
            args.as_slice()
        );
    }

    #[test]
    fn test_notify_vmexit_disable_and_notify_window() {
        let mut qemu = Qemu::<Generic>::new();
        qemu.accel(Accel {
            accelerator: Accelerator::Kvm,
            notify_vmexit: Some(NotifyVmexit::DisableAndNotifyWindow(123456789)),
            ..Default::default()
        });

        let args = qemu.args();
        assert_eq!(
            &[
                "-accel",
                "accel=kvm,idg-passthru=off,kernel-irqchip=on,notify-vmexit=disable,notify-window=123456789"
            ],
            args.as_slice()
        );
    }
}
