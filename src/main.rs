mod config;
mod deploy;
mod project;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{env, rc::Rc};

use crate::{
    config::Config,
    deploy::{BuildProvider, DefaultDeploy, DeployProvider},
    project::{DefaultManifest, DefaultProject, ManifestProvider, ProjectProvider, Target},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Build {
        #[clap(short, long)]
        target: Target,
        #[clap(long)]
        release: bool,
        #[clap(long)]
        cargo_build: Option<String>,
    },
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "magisk" {
        args.remove(1);
    }

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Build {
            target,
            release,
            cargo_build,
        } => {
            build_cmd(target, release, cargo_build)?;
        }
    }
    Ok(())
}

fn build_cmd(target: Target, release: bool, cargo_build: Option<String>) -> Result<()> {
    let manifest_provider: Rc<dyn ManifestProvider> = Rc::new(DefaultManifest::new());
    let project_provider: Rc<dyn ProjectProvider> = Rc::new(DefaultProject::new(
        target.clone(),
        release,
        manifest_provider.clone(),
    ));
    let config = Config::load(&manifest_provider, &project_provider)?;
    let deploy = DefaultDeploy::new(project_provider);

    deploy.build(cargo_build)?;
    deploy.deploy(&config)?;

    Ok(())
}
