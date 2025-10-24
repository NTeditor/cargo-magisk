#[cfg(test)]
mod test;
mod toml_types;

use crate::project::{ManifestProvider, ProjectProvider};
use anyhow::{Context, Result, bail};
use regex::Regex;
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
    const VERSION_MAJOR_FACTOR: u64 = 100_000_000;
    const VERSION_MINOR_FACTOR: u64 = 1_000_000;
    const VERSION_PATCH_FACTOR: u64 = 10_000;
    const VERSION_RELEASE_TYPE_FACTOR: u64 = 100;

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
            bail!("Invalid id: value is empty");
        }

        let re_id = Regex::new(r"^[a-zA-Z][a-zA-Z0-9._-]+$")?;
        if !re_id.is_match(id) {
            bail!("Invalid id: unsupported format");
        }

        if name.is_empty() {
            bail!("Invalid name: value is empty");
        }

        if version.is_empty() {
            bail!("Invalid version: value is empty");
        }

        let re_version =
            Regex::new(r"^([0-9]|[1-9]\d)\.([0-9]|[1-9]\d)\.([0-9]|[1-9]\d)(?:-([\w.-]+))?$")?;
        if !re_version.is_match(version) {
            bail!("Invalid version: unsupported format");
        }

        if author.is_empty() {
            bail!("Invalid author: value is empty");
        }

        Ok(())
    }

    fn parse_version(version: &str) -> Result<(String, u64)> {
        let re = Regex::new(
            r"^(?P<major>\d{1,2})\.(?P<minor>\d{1,2})\.(?P<patch>\d{1,2})(?:-(?P<pre_type>alpha|beta|rc)(?:\.(?P<pre_code>\d{1,2}))?)?$",
        )?;

        let caps = re.captures(version).context("Invalid version format")?;
        let major: u64 = caps["major"].parse().context("Invalid major version")?;
        let minor: u64 = caps["minor"].parse().context("Invalid minor version")?;
        let patch: u64 = caps["patch"].parse().context("Invalid patch version")?;
        let mut version_code = major * Self::VERSION_MAJOR_FACTOR
            + minor * Self::VERSION_MINOR_FACTOR
            + patch * Self::VERSION_PATCH_FACTOR;

        if let Some(pre_type_str) = caps.name("pre_type") {
            let pre_type =
                ReleaseType::try_from(pre_type_str.as_str()).context("Invalid pre-type")?;
            version_code += pre_type as u64 * Self::VERSION_RELEASE_TYPE_FACTOR;
        } else {
            let pre_type = ReleaseType::Stable;
            version_code += pre_type as u64 * Self::VERSION_RELEASE_TYPE_FACTOR;
        }

        if let Some(pre_code_str) = caps.name("pre_code") {
            let pre_code: u64 = pre_code_str.as_str().parse().context("Invalid pre-type")?;
            version_code += pre_code;
        }

        Ok((version.to_string(), version_code))
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
        let source = Self::parse_source(source, provider).context("Failed to initialize Asset")?;
        let dest = Self::parse_dest(dest, provider).context("Failed to initialize Asset")?;
        Ok(Self { source, dest })
    }

    fn parse_source(source: String, provider: &Rc<dyn ProjectProvider>) -> Result<PathBuf> {
        if source.is_empty() {
            bail!("Invalid source: value is empty");
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
            bail!("Invalid dest: value is empty");
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
