use qemu_api::args::{Accel, Accelerator, Cache, Drive, Format};
use qemu_api::{Qemu, X86_64};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut qemu = Qemu::<X86_64>::new();
    qemu.accel(Accel {
        accelerator: Accelerator::Kvm,
        ..Default::default()
    });
    qemu.bios(PathBuf::from("hello/world"));
    qemu.drive(Drive {
        file: PathBuf::from("other/path.img"),
        format: Some(Format::Raw),
        ..Default::default()
    });
    qemu.drive(Drive {
        file: PathBuf::from("yet/another/path.img"),
        format: Some(Format::Raw),
        cache: Some(Cache::Directsync),
        ..Default::default()
    });

    let cmd = Command::from(qemu);
    println!("cmd: {:?}", cmd);
}
