#[cfg(test)]
mod test;

use std::path::PathBuf;

use anyhow::{Result, bail};

trait ProjectProvider {
    fn get_project_path(&self) -> Result<PathBuf>;
}

pub struct DefaultProvider {
    target: Target,
    release: bool,
    provider: Box<dyn ManifestProvider>,
}

trait ManifestProvider {
    fn find_manifest_path(&self) -> Result<PathBuf>;
}

pub struct DefaultManifest;

impl ManifestProvider for DefaultManifest {
    fn find_manifest_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::new())
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
