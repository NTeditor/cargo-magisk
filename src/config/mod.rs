#[cfg(test)]
mod test;
mod toml_types;
mod version_code;

use crate::project::{ManifestProvider, ProjectProvider};
use anyhow::{Context, Result, bail};
use regex::Regex;
use std::{
    fmt::Display,
    fs,
    path::{Component, Path, PathBuf},
    rc::Rc,
};
use version_code::VersionCode;

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
        Self::load_with_path_unchecked(manifest_path, project_provider)
    }

    pub fn load_with_path(
        manifest_path: PathBuf,
        project_provider: &Rc<dyn ProjectProvider>,
    ) -> Result<Self> {
        if !manifest_path.exists() {
            bail!("Invalid manifest path: file not exists");
        }

        Self::load_with_path_unchecked(manifest_path, project_provider)
    }

    fn load_with_path_unchecked(
        manifest_path: PathBuf,
        project_provider: &Rc<dyn ProjectProvider>,
    ) -> Result<Self> {
        let manifest_content =
            fs::read_to_string(manifest_path).context("Failed read Cargo.toml")?;
        let config: toml_types::Manifest =
            toml::from_str(&manifest_content).context("Invalid Cargo.toml: failed parse")?;

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
    pub version_code: VersionCode,
    pub author: String,
}

impl ModuleProp {
    pub fn new(id: String, name: String, version: String, author: String) -> Result<Self> {
        Self::validate(&id, &name, &version, &author)?;
        let version_code = VersionCode::try_from(version.as_str())?;

        Ok(Self {
            id,
            name,
            version,
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

        if author.is_empty() {
            bail!("Invalid author: value is empty");
        }

        Ok(())
    }
}

impl Display for ModuleProp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "id={}\n\
             name={}\n\
             author={}\n\
             version={}\n\
             versionCode={}",
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

        let source_path = Path::new(&source);
        Self::check_path(source_path, "source")?;

        let result = match source_path.strip_prefix("target") {
            Ok(value) => {
                let mut target_path = provider.get_target_path()?;
                target_path.push(value);
                target_path
            }
            Err(_) => {
                let mut project_path = provider.get_project_path()?;
                project_path.push(source_path);
                project_path
            }
        };

        Ok(result)
    }

    fn parse_dest(dest: String, provider: &Rc<dyn ProjectProvider>) -> Result<PathBuf> {
        if dest.is_empty() {
            bail!("Invalid dest: value is empty");
        }

        let dest_path = Path::new(&dest);
        Self::check_path(dest_path, "dest")?;

        let mut target_path = provider.get_target_path()?;
        target_path.push("magisk");
        target_path.push(dest_path);
        Ok(target_path)
    }

    fn check_path(path: &Path, label: &str) -> Result<()> {
        if path.is_absolute() {
            bail!("Invalid {}: path is absolute", label);
        }

        for comp in path.components() {
            match comp {
                Component::ParentDir => bail!("Invalid {}: contains '..'", label),
                Component::CurDir => bail!("Invalid {}: contains '.'", label),
                Component::Normal(name) if name == "..." => {
                    bail!("Invalid {}: contains '...'", label)
                }
                _ => {}
            }
        }
        Ok(())
    }
}
