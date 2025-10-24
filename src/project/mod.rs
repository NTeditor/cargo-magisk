#[cfg(test)]
mod test;

use std::{
    env,
    fmt::{Debug, Display},
    path::PathBuf,
    rc::Rc,
};

use anyhow::{Ok, Result, bail};

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

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Arm64V8a,
    ArmeabiV7a,
    X86_64,
    X86,
}

impl Target {
    pub const ARM64_V8A_STR: &str = "aarch64-linux-android";
    pub const ARMEABI_V7A_STR: &str = "armv7-linux-androideabi";
    pub const X86_64_STR: &str = "x86_64-linux-android";
    pub const X86_STR: &str = "i686-linux-android";
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::Arm64V8a => Self::ARM64_V8A_STR,
            Self::ArmeabiV7a => Self::ARMEABI_V7A_STR,
            Self::X86_64 => Self::X86_64_STR,
            Self::X86 => Self::X86_STR,
        };
        write!(f, "{}", result)
    }
}

impl TryFrom<&str> for Target {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            Self::ARM64_V8A_STR => Ok(Self::Arm64V8a),
            Self::ARMEABI_V7A_STR => Ok(Self::ArmeabiV7a),
            Self::X86_64_STR => Ok(Self::X86_64),
            Self::X86_STR => Ok(Self::X86),
            _ => bail!("Invaid target"),
        }
    }
}
