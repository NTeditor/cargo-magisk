use std::path::Path;

use crate::project::Target;

use super::*;
use anyhow::Result;
use rstest::rstest;

#[rstest]
#[case(
    "target/cargo-magisk",
    "system/bin/cargo-magisk",
    Path::new("/workspace/target/arch/build_type/cargo-magisk"),
    Path::new("/workspace/target/arch/build_type/magisk/system/bin/cargo-magisk")
)]
#[case(
    "assets/customize.sh",
    "customize.sh",
    Path::new("/workspace/assets/customize.sh"),
    Path::new("/workspace/target/arch/build_type/magisk/customize.sh")
)]
fn asset_parse(
    #[case] source: String,
    #[case] dest: String,
    #[case] expenced_source: &Path,
    #[case] expenced_dest: &Path,
) {
    let provider: Rc<dyn ProjectProvider> = Rc::new(MockProject {});
    let asset = Asset::try_new(source, dest, &provider).unwrap();
    assert_eq!(&asset.source, expenced_source);
    assert_eq!(&asset.dest, expenced_dest);
}

#[derive(Debug)]
struct MockProject;
impl ProjectProvider for MockProject {
    fn get_project_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/workspace"))
    }

    fn get_target_path(&self) -> Result<PathBuf> {
        let mut project_path = self.get_project_path().unwrap();
        project_path.push("target/arch/build_type");
        Ok(project_path)
    }

    fn get_target(&self) -> &Target {
        &Target::Arm64V8a
    }

    fn is_release(&self) -> bool {
        false
    }
}
