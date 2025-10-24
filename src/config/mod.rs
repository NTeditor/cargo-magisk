#[cfg(test)]
mod test;
mod toml_types;

use crate::project::{ManifestProvider, ProjectProvider};
use anyhow::{Context, Result, bail};
use regex::Regex;
use semver::Version;
use std::{fmt::Display, fs, path::PathBuf, rc::Rc};

#[derive(Debug)]
pub struct Config {
    pub module_prop: ModuleProp,
    pub assets: Vec<Asset>,
}

impl Config {
    pub fn load(
        manifest_provider: &Rc<dyn ManifestProvider>,
        project_provider: &Rc<dyn ProjectProvider>,
    ) -> Result<Self> {
        let manifest_path = manifest_provider.find_manifest_path()?;
        let manifest_content = fs::read_to_string(manifest_path)?;
        let config: toml_types::Manifest = toml::from_str(&manifest_content)?;

        let module_prop = ModuleProp::new(
            config.package.metadata.magisk.id,
            config.package.metadata.magisk.name,
            config.package.version,
            config.package.metadata.magisk.author,
        )?;
        let mut assets: Vec<Asset> = vec![];
        for asset in config.package.metadata.magisk.assets {
            assets.push(Asset::try_new(asset.source, asset.dest, project_provider)?);
        }

        Ok(Self {
            module_prop,
            assets,
        })
    }
}

#[derive(Debug)]
pub struct ModuleProp {
    pub id: String,
    pub name: String,
    pub version: String,
    pub version_code: u64,
    pub author: String,
}

impl ModuleProp {
    pub fn new(id: String, name: String, version: String, author: String) -> Result<Self> {
        Self::validate(&id, &name, &version, &author)?;
        let (version_valid, version_code) = Self::parse_version(&version)?;

        Ok(Self {
            id,
            name,
            version: version_valid,
            version_code,
            author,
        })
    }

    fn validate(id: &str, name: &str, version: &str, author: &str) -> Result<()> {
        if id.is_empty() {
            bail!("'package.manifest.magisk.id' is empty");
        }

        let re = Regex::new("^[a-zA-Z][a-zA-Z0-9._-]+$")?;
        if !re.is_match(id) {
            bail!("Invalid 'package.manifest.magisk.id'");
        }

        if name.is_empty() {
            bail!("'package.manifest.magisk.name' is empty");
        }

        if version.is_empty() {
            bail!("'package.manifest.magisk.version' is empty");
        }

        if author.is_empty() {
            bail!("'package.manifest.magisk.author' is empty");
        }

        Ok(())
    }

    fn parse_version(version: &str) -> Result<(String, u64)> {
        let ver = Version::parse(version)?;
        let mut version_code = 0;
        version_code += ver.major * 100_000_000;
        version_code += ver.minor * 1_000_000;
        version_code += ver.patch * 10_000;
        if ver.pre.is_empty() {
            version_code += ReleaseType::Stable as u64 * 100;
        } else {
            let pre_release_str = ver.pre.as_str();
            let identifiers: Vec<&str> = pre_release_str.split('.').collect();
            if identifiers.len() != 1 && identifiers.len() != 2 {
                bail!("Invaild pre-version");
            }
            let type_str = identifiers[0];
            let release_type = ReleaseType::try_from(type_str)?;
            version_code += release_type as u64 * 100;
            if let Some(value) = identifiers.get(1) {
                let pre_code: u64 = value.parse().context("Pre-release number not u64")?;
                version_code += pre_code;
            }
        }
        Ok((ver.to_string(), version_code))
    }
}

impl Display for ModuleProp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id={}\n\
             name={}\n\
             author={}\n\
             version={}\n\
             versionCode={}\n",
            self.id, self.name, self.author, self.version, self.version_code
        )
    }
}

#[derive(Debug)]
pub struct Asset {
    pub source: PathBuf,
    pub dest: PathBuf,
}

impl Asset {
    pub fn try_new(
        source: String,
        dest: String,
        provider: &Rc<dyn ProjectProvider>,
    ) -> Result<Self> {
        let source = Self::parse_source(source, provider)?;
        let dest = Self::parse_dest(dest, provider)?;
        Ok(Self { source, dest })
    }

    fn parse_source(source: String, provider: &Rc<dyn ProjectProvider>) -> Result<PathBuf> {
        if source.is_empty() {
            bail!("source in Asset is empty");
        }
        let result = match source.starts_with("target") {
            true => {
                let mut target_path = provider.get_target_path()?;
                let source_replaced = source.replace("target/", "");
                target_path.push(source_replaced);
                target_path
            }
            false => {
                let mut project_path = provider.get_project_path()?;
                project_path.push(source);
                project_path
            }
        };
        Ok(result)
    }

    fn parse_dest(dest: String, provider: &Rc<dyn ProjectProvider>) -> Result<PathBuf> {
        if dest.is_empty() {
            bail!("dest in Asset is empty");
        }
        let mut target_path = provider.get_target_path()?;
        target_path.push("magisk");
        target_path.push(dest);
        Ok(target_path)
    }
}

#[derive(Debug)]
#[repr(u64)]
enum ReleaseType {
    Alpha = 1,
    Beta = 2,
    Rc = 3,
    Stable = 9,
}

impl TryFrom<&str> for ReleaseType {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "alpha" => Ok(ReleaseType::Alpha),
            "beta" => Ok(ReleaseType::Beta),
            "rc" => Ok(ReleaseType::Rc),
            _ => bail!("Invalid build type: {}", name),
        }
    }
}
