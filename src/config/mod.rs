#[cfg(test)]
mod test;
mod toml_types;

use crate::project::{ManifestProvider, ProjectProvider};
use anyhow::Result;
use std::{fmt::Display, fs, path::PathBuf, rc::Rc};

#[derive(Debug)]
pub struct Config {
    pub module_prop: ModuleProp,
    pub assets: Vec<Asset>,
}

impl Config {
    pub fn load(
        manifest_provider: Rc<dyn ManifestProvider>,
        project_provider: Rc<dyn ProjectProvider>,
    ) -> Result<Self> {
        let manifest_path = manifest_provider.find_manifest_path()?;
        let manifest_content = fs::read_to_string(manifest_path)?;
        let config: toml_types::Manifest = toml::from_str(&manifest_content)?;

        let module_prop = ModuleProp {
            id: config.package.metadata.magisk.id,
            name: config.package.metadata.magisk.name,
            version: config.package.version,
            version_code: 0,
            author: config.package.metadata.magisk.author,
        };

        let mut assets: Vec<Asset> = vec![];
        for asset in config.package.metadata.magisk.assets {
            assets.push(Asset::try_new(asset.source, asset.dest, &project_provider)?);
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
        let mut target_path = provider.get_target_path()?;
        target_path.push("magisk");
        target_path.push(dest);
        Ok(target_path)
    }
}
