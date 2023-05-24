use crate::args::QemuArgument;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LogItem {
    OutAsm,
    InAsm,
    Op,
    OpOpt,
    OpInd,
    Int,
    Exec,
    Cpu,
    Fpu,
    Mmu,
    Pcall,
    CpuReset,
    Unimp,
    GuestErrors,
    Page,
    Nochain,
    Plugin,
    Strace,
    Tid,
    Trace(String),
    Help,
}

impl LogItem {
    fn format(&self) -> String {
        match self {
            LogItem::OutAsm => "out_asm".to_string(),
            LogItem::InAsm => "in_asm".to_string(),
            LogItem::Op => "op".to_string(),
            LogItem::OpOpt => "op_opt".to_string(),
            LogItem::OpInd => "op_ind".to_string(),
            LogItem::Int => "int".to_string(),
            LogItem::Exec => "exec".to_string(),
            LogItem::Cpu => "cpu".to_string(),
            LogItem::Fpu => "fpu".to_string(),
            LogItem::Mmu => "mmu".to_string(),
            LogItem::Pcall => "pcall".to_string(),
            LogItem::CpuReset => "cpu_reset".to_string(),
            LogItem::Unimp => "unimp".to_string(),
            LogItem::GuestErrors => "guest_errors".to_string(),
            LogItem::Page => "page".to_string(),
            LogItem::Nochain => "nochain".to_string(),
            LogItem::Plugin => "plugin".to_string(),
            LogItem::Strace => "strace".to_string(),
            LogItem::Tid => "tid".to_string(),
            LogItem::Trace(pattern) => format!("trace:{}", pattern),
            LogItem::Help => "help".to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct LogItems {
    pub items: Vec<LogItem>,
}

impl<I> From<I> for LogItems
where
    I: IntoIterator<Item = LogItem>,
{
    fn from(value: I) -> Self {
        Self {
            items: value.into_iter().collect(),
        }
    }
}

impl QemuArgument for LogItems {
    fn format(&self) -> Vec<String> {
        let mut arg = String::new();

        if let Some(first) = self.items.first() {
            arg.push_str(first.format().as_str())
        }
        self.items.iter().skip(1).for_each(|i| {
            arg.push(',');
            arg.push_str(i.format().as_str());
        });

        vec!["-d".to_string(), arg]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_items_from_into_iter() {
        let items = LogItems::from([LogItem::Page, LogItem::GuestErrors]);
        let inner = items.items;
        assert_eq!(2, inner.len());
        assert!(inner.contains(&LogItem::Page));
        assert!(inner.contains(&LogItem::GuestErrors));
    }
}
