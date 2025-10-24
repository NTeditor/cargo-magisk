use crate::config::Config;
use crate::project::{ProjectProvider, Target};
use anyhow::{Result, bail};
use std::fmt::Debug;
use std::fs;
use std::process::Command;
use std::rc::Rc;

pub trait Deploy: Debug {
    fn deploy(&self, config: &Config) -> Result<()>;
}

trait Build: Debug {
    fn build(&self, target: &Target, release: bool, cargo_build: Option<String>) -> Result<()>;
}

#[derive(Debug)]
pub struct DefaultDeploy {
    cargo_build: Option<String>,
    project_provider: Rc<dyn ProjectProvider>,
    build: Box<dyn Build>,
}

impl DefaultDeploy {
    pub fn new(project_provider: Rc<dyn ProjectProvider>, cargo_build: Option<String>) -> Self {
        let build: Box<dyn Build> = Box::new(BuildShell::new());
        Self {
            cargo_build,
            project_provider,
            build,
        }
    }

    fn clean(&self) -> Result<()> {
        let mut project_path = self.project_provider.get_target_path()?;
        project_path.push("magisk");

        if project_path.exists() {
            fs::remove_dir_all(project_path)?;
        }
        Ok(())
    }
}

impl Deploy for DefaultDeploy {
    fn deploy(&self, config: &Config) -> Result<()> {
        self.clean()?;
        self.build.build(
            self.project_provider.get_target(),
            self.project_provider.is_release(),
            self.cargo_build.clone(),
        )?;
        for asset in &config.assets {
            let source = &asset.source;
            let dest = &asset.dest;

            if !source.exists() {
                bail!("Asset source not found: '{}'", asset.source.display());
            }

            match dest.parent() {
                Some(value) => {
                    fs::create_dir_all(value)?;
                }
                None => {
                    bail!("Asset dest failed get parent: '{}'", dest.display());
                }
            }

            if source.is_file() {
                fs::copy(source, dest)?;
            }

            if source.is_dir() {
                let copy_options = fs_extra::dir::CopyOptions::new();
                fs_extra::dir::copy(source, dest, &copy_options)?;
            }
        }
        let module_prop_string = config.module_prop.to_string();
        self.write_module_prop(&module_prop_string)?;
        Ok(())
    }
}

impl DefaultDeploy {
    fn write_module_prop(&self, content: &str) -> Result<()> {
        let mut module_prop_path = self.project_provider.get_target_path()?.join("magisk");
        module_prop_path.push("module.prop");
        fs::write(module_prop_path, content)?;
        Ok(())
    }
}

#[derive(Debug)]
struct BuildShell;
impl BuildShell {
    pub fn new() -> Self {
        Self {}
    }
}

impl Build for BuildShell {
    fn build(&self, target: &Target, release: bool, cargo_build: Option<String>) -> Result<()> {
        let mut proc = Command::new("cargo");
        if let Some(value) = cargo_build {
            proc.arg(value);
        }

        proc.args(["build", "--target", &target.to_string()]);
        if release {
            proc.arg("--release");
        }

        println!("Building..");
        println!("---------------------");
        let mut child = proc.spawn()?;
        child.wait()?;
        println!("---------------------");
        println!("Done");
        Ok(())
    }
}
