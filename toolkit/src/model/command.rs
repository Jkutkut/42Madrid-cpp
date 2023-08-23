#[derive(Debug)]
pub enum Command {
    NewProject,
    NewExercise,
}

impl Command {
    pub fn from(s: &str) -> Result<Self, String> {
        match s {
            "new_project" => Ok(Command::NewProject),
            "new_exercise" => Ok(Command::NewExercise),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}
