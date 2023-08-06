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
        Ok(Self {
            working_directory,
            starting_command,
            starting_command_arguments,
        })
    }

    pub fn create(&self) -> Result<()> {
        std::fs::create_dir_all(&self.working_directory)?;

        if !Command::new("python")
            .args(vec!["-m", "venv", "env"])
            .current_dir(&self.working_directory)
            .status()?
            .success() {
            anyhow::bail!("Failed to create virtual environment")
        }

        if !Command::new("sh")
            .args(vec!["-c", ". env/bin/activate && pip install nonebot2[fastapi]"])
            .current_dir(&self.working_directory)
            .status()?
            .success() {
            anyhow::bail!("Failed to install NoneBot 2")
        }

        Ok(())
    }

    pub fn start(&self) -> Result<bool> {
        Ok(Command::new(&self.starting_command)
            .args(&self.starting_command_arguments)
            .current_dir(&self.working_directory)
            .status()?
            .success())
    }
}

#[cfg(test)]
mod tests {
    use super::Instance;

    use std::path::Path;

    #[test]
    fn create_instance() {
        let instance = Instance::new(
            "test-instance".to_string(),
            "touch".to_string(),
            vec!["RUNNING".to_string()],
        )
        .unwrap();
        instance.create().unwrap();
        assert!(Path::new("test-instance").exists());
    }
}
