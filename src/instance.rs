use std::process::Command;

use std::io::Write;

use anyhow::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    working_directory: String,
}

impl Instance {
    pub fn new(working_directory: String) -> Result<Self> {
        Ok(Self { working_directory })
    }

    pub fn create(&self) -> Result<()> {
        std::fs::create_dir_all(&self.working_directory)?;

        if !Command::new("python")
            .args(vec!["-m", "venv", "env"])
            .current_dir(&self.working_directory)
            .status()?
            .success()
        {
            anyhow::bail!("Failed to create virtual environment")
        }

        if !Command::new("sh")
            .args(vec![
                "-c",
                ". env/bin/activate && pip install nonebot2[fastapi]",
            ])
            .current_dir(&self.working_directory)
            .status()?
            .success()
        {
            anyhow::bail!("Failed to install NoneBot 2")
        }

        std::fs::File::create(format!("{}/bot.py", self.working_directory))?
            .write_all(include_bytes!("bot.py"))?;

        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        let mut child = Command::new("sh")
            .args(vec!["-c", ". env/bin/activate && python bot.py"])
            .current_dir(&self.working_directory)
            .spawn()?;
        std::thread::spawn(move || {
            child.wait().unwrap();
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Instance;

    use std::path::Path;

    #[test]
    fn create_instance() {
        let instance = Instance::new("test-instance".to_string()).unwrap();
        instance.create().unwrap();
        assert!(Path::new("test-instance").exists());
        assert!(Path::new("test-instance/bot.py").exists());
    }
}
