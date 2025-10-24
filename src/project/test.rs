use std::path::Path;

use super::*;
use rstest::rstest;

#[rstest]
#[case(Path::new("/workspace"))]
fn default_project_get_project_path(#[case] expected: &Path) {
    let provider = Rc::new(MockManifest {});
    let project = DefaultProject::new(Target::Arm64V8a, true, provider);
    let project_path = project.get_project_path().unwrap();
    assert_eq!(&project_path, expected);
}

#[rstest]
#[case(
    Target::Arm64V8a,
    true,
    Path::new("/workspace/target/aarch64-linux-android/release")
)]
#[case(
    Target::Arm64V8a,
    false,
    Path::new("/workspace/target/aarch64-linux-android/debug")
)]
#[case(
    Target::ArmeabiV7a,
    true,
    Path::new("/workspace/target/armv7-linux-androideabi/release")
)]
#[case(
    Target::ArmeabiV7a,
    false,
    Path::new("/workspace/target/armv7-linux-androideabi/debug")
)]
#[case(
    Target::X86_64,
    true,
    Path::new("/workspace/target/x86_64-linux-android/release")
)]
#[case(
    Target::X86_64,
    false,
    Path::new("/workspace/target/x86_64-linux-android/debug")
)]
#[case(
    Target::X86,
    true,
    Path::new("/workspace/target/i686-linux-android/release")
)]
#[case(
    Target::X86,
    false,
    Path::new("/workspace/target/i686-linux-android/debug")
)]
fn default_project_get_target_path(
    #[case] target: Target,
    #[case] release: bool,
    #[case] expected: &Path,
) {
    let provider = Rc::new(MockManifest {});
    let project = DefaultProject::new(target, release, provider);
    let target_path = project.get_target_path().unwrap();
    assert_eq!(&target_path, expected);
}

#[derive(Debug)]
struct MockManifest;
impl ManifestProvider for MockManifest {
    fn find_manifest_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/workspace/Cargo.toml"))
    }
}
