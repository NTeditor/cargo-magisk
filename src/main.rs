mod config;
mod deploy;
mod project;

use std::rc::Rc;

use crate::{
    deploy::{BuildProvider, DeployProvider},
    project::{DefaultManifest, DefaultProject, ManifestProvider, ProjectProvider, Target},
};
use anyhow::Result;

fn main() -> Result<()> {
    let manifest_provider: Rc<dyn ManifestProvider> = Rc::new(DefaultManifest::new());
    let project_provider: Rc<dyn ProjectProvider> = Rc::new(DefaultProject::new(
        Target::Arm64V8a,
        false,
        manifest_provider.clone(),
    ));

    let config = config::Config::load(manifest_provider.clone(), project_provider.clone())?;
    let deploy = deploy::DefaultDeploy::new(Target::Arm64V8a, false, project_provider.clone());
    deploy.build(Some("ndk"))?;
    deploy.deploy(&config)?;
    println!("{:#?}", config);
    Ok(())
}
