use crate::args::QemuArgument;
use std::path::PathBuf;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Drive {
    pub file: PathBuf,
    pub format: Option<Format>,
    pub cache: Option<Cache>,
    pub snapshot: Option<Snapshot>,
}

impl QemuArgument for Drive {
    fn format(&self) -> Vec<String> {
        let mut arg = format!("file={}", self.file.display());
        if let Some(format) = self.format {
            arg.push_str(&format!(",format={}", format.name()));
        }
        if let Some(cache) = self.cache {
            arg.push_str(&format!(",cache={}", cache.name()));
        }
        if let Some(snapshot) = self.snapshot {
            arg.push_str(&format!(",snapshot={}", snapshot.name()));
        }
        vec!["-drive".to_string(), arg]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Format {
    Raw,
}

impl Format {
    fn name(&self) -> &'static str {
        match self {
            Self::Raw => "raw",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cache {
    Writethrough,
    Writeback,
    None,
    Directsync,
    Unsafe,
}

impl Cache {
    fn name(&self) -> &'static str {
        match self {
            Self::Writethrough => "writethrough",
            Self::Writeback => "writeback",
            Self::None => "none",
            Self::Directsync => "directsync",
            Self::Unsafe => "unsafe",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Snapshot {
    On,
    Off,
}

impl Snapshot {
    fn name(&self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
    }
}
