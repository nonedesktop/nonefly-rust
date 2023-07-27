use std::process::Command;

use anyhow::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    working_directory: String,
    starting_command: String,
    starting_command_arguments: Vec<String>,
}

impl Instance {
    pub fn new(
        working_directory: String,
        starting_command: String,
        starting_command_arguments: Vec<String>,
    ) -> Result<Self> {
        std::fs::create_dir_all(&working_directory)?;

        Ok(Self {
            working_directory,
            starting_command,
            starting_command_arguments,
        })
    }

    pub fn start(&self) -> Result<bool> {
        std::fs::create_dir_all(&self.working_directory)?;

        Ok(Command::new(&self.starting_command)
            .args(&self.starting_command_arguments)
            .current_dir(&self.working_directory)
            .status()?
            .success())
    }
}
