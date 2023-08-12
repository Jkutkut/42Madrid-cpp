#[derive(Debug)]
pub enum Command {
    NewProject,
}

impl Command {
    pub fn from(s: &str) -> Result<Self, String> {
        match s {
            "new_project" => Ok(Command::NewProject),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}
