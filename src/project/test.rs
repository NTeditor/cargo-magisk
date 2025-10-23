use std::path::Path;

use super::*;
use rstest::rstest;

#[rstest]
#[case(Target::ARM64_V8A_STR, Target::Arm64V8a)]
fn target_enum_from(#[case] target_string: &str, #[case] expected: Target) {
    let target = Target::try_from(target_string).unwrap();
    assert_eq!(target, expected);
}

#[rstest]
#[case(Target::Arm64V8a, Target::ARM64_V8A_STR)]
fn target_enum_to_string(#[case] target: Target, #[case] expected: &str) {
    let target_str = target.to_string();
    assert_eq!(&target_str, expected);
}

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
