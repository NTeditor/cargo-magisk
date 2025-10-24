#[cfg(test)]
mod test;

use std::{
    env,
    fmt::{Debug, Display},
    path::PathBuf,
    rc::Rc,
};

use anyhow::{Ok, Result, bail};
use clap::ValueEnum;

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

impl DefaultProject {
    pub fn new(target: Target, release: bool, provider: Rc<dyn ManifestProvider>) -> Self {
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

#[derive(Clone, Debug)]
pub struct DefaultManifest;

impl DefaultManifest {
    pub fn new() -> Self {
        Self {}
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

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Target {
    #[value(name = "aarch64-linux-android", alias = "arm64-v8a")]
    Arm64V8a,
    #[value(name = "armv7-linux-androideabi", alias = "armeabi-v7a")]
    ArmeabiV7a,
    #[value(name = "x86_64-linux-android", alias = "x64_64")]
    X86_64,
    #[value(name = "x86_64-linux-android", alias = "x86")]
    X86,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_possible_value() {
            Some(value) => write!(f, "{}", value.get_name()),
            None => Err(std::fmt::Error),
        }
    }
}
