use anyhow::Result;

use crate::project::Target;

mod config;
mod project;

fn main() -> Result<()> {
    let config = config::Config::load(Target::Arm64V8a, true)?;
    println!("{:#?}", config);
    drop(config);
    Ok(())
}
