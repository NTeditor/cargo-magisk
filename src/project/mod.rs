#[cfg(test)]
mod test;

use std::{env, fmt::Debug, path::PathBuf, rc::Rc};

use anyhow::{Result, bail};

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Arm64V8a,
}

pub trait ProjectProvider: Debug {
    fn get_project_path(&self) -> Result<PathBuf>;
    fn get_target_path(&self) -> Result<PathBuf>;
}

pub trait ManifestProvider: Debug {
    fn find_manifest_path(&self) -> Result<PathBuf>;
}

#[derive(Debug, Clone)]
pub struct DefaultProject {
    target: Target,
    release: bool,
    provider: Rc<dyn ManifestProvider>,
}

#[derive(Clone, Debug)]
pub struct DefaultManifest;

impl Target {
    pub const ARM64_V8A_STR: &str = "aarch64-linux-android";
}

impl DefaultProject {
    pub fn new(target: Target, release: bool, provider: Rc<dyn ManifestProvider>) -> Self {
        Self {
            target,
            release,
            provider,
        }
    }
}

impl DefaultManifest {
    pub fn new() -> Self {
        Self {}
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

impl ManifestProvider for DefaultManifest {
    fn find_manifest_path(&self) -> Result<PathBuf> {
        let mut current_dir = env::current_dir()?;
        loop {
            let manifest_path = current_dir.join("Cargo.toml");
            if manifest_path.exists() {
                return Ok(manifest_path);
            }

            if !current_dir.pop() {
                bail!("Cargo.toml not found");
            }
        }
    }
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Self::Arm64V8a => Self::ARM64_V8A_STR.to_string(),
        }
    }
}

impl TryFrom<&str> for Target {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            Self::ARM64_V8A_STR => {
                return Ok(Self::Arm64V8a);
            }
            _ => {
                bail!("Invaid target");
            }
        }
    }
}
