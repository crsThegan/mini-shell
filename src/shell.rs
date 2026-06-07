use super::ExecuteError;

pub enum ShellMode {
    OneLiner(String),
    Full,
    File(String),
}

pub struct Shell {
    pub mode: ShellMode,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            mode: ShellMode::Full,
        }
    }

    pub fn from_mode(mode: ShellMode) -> Self {
        Shell { mode }
    }

    pub fn start(&self) {
        todo!()
    }

    pub fn execute(&self, cmd: &str) -> Result<(), ExecuteError> {
        todo!()
    }
}
