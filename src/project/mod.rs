#[cfg(test)]
mod test;

use std::{env, path::PathBuf};

use anyhow::{Result, bail};

trait ProjectProvider {
    fn get_project_path(&self) -> Result<PathBuf>;
    fn get_target_path(&self) -> Result<PathBuf>;
}

pub struct DefaultProject {
    target: Target,
    release: bool,
    provider: Box<dyn ManifestProvider>,
}

impl DefaultProject {
    fn new(target: Target, release: bool, provider: Box<dyn ManifestProvider>) -> Self {
        Self {
            target,
            release,
            provider,
        }
    }
}

impl ProjectProvider for DefaultProject {
    fn get_project_path(&self) -> Result<PathBuf> {
        let mut result = self.provider.find_manifest_path()?;
        if !result.pop() {
            bail!("Failed get Cargo.toml parent");
        }
        Ok(result)
    }

    fn get_target_path(&self) -> Result<PathBuf> {
        let mut result = self.get_project_path()?;
        result.push("target");
        result.push(self.target.to_string());
        let build_type = if self.release { "release" } else { "debug" };
        result.push(build_type);
        Ok(result)
    }
}

trait ManifestProvider {
    fn find_manifest_path(&self) -> Result<PathBuf>;
}

pub struct DefaultManifest;
impl DefaultManifest {
    fn new() -> Self {
        Self {}
    }
}

impl ManifestProvider for DefaultManifest {
    fn find_manifest_path(&self) -> Result<PathBuf> {
        let current_dir = env::current_dir()?;
        let mut result = current_dir.join("Cargo.toml");
        loop {
            if result.exists() {
                return Ok(result.to_path_buf());
            }

            if !result.pop() {
                bail!("Cargo.toml not found");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Arm64V8a,
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Self::Arm64V8a => {
                return "aarch64-linux-android".to_string();
            }
        }
    }
}

impl TryFrom<&str> for Target {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "aarch64-linux-android" => {
                return Ok(Self::Arm64V8a);
            }
            _ => {
                bail!("Invaid target");
            }
        }
    }
}
