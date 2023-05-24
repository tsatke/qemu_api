#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum QemuCharDevice {
    Stdio,
    Id(String),
    Help,
}

impl QemuCharDevice {
    pub fn format(&self) -> String {
        match self {
            Self::Stdio => "stdio".to_string(),
            QemuCharDevice::Id(id) => id.clone(),
            QemuCharDevice::Help => "help".to_string(),
        }
    }
}
