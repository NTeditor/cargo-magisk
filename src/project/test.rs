use super::*;
use rstest::rstest;

#[rstest]
#[case("aarch64-linux-android", Target::Arm64V8a)]
fn target_enum_from(#[case] target_string: String, #[case] expected: Target) {
    let target = Target::try_from(target_string.as_str()).unwrap();
    assert_eq!(target, expected);
}

#[rstest]
#[case(Target::Arm64V8a, "aarch64-linux-android")]
fn target_enum_to_string(#[case] target: Target, #[case] expected: String) {
    let target_str = target.to_string();
    assert_eq!(target_str, expected);
}

#[rstest]
fn default_project_get_project_path() {
    let provider = Box::new(MockManifest {});
    let project = DefaultProject::new(Target::Arm64V8a, true, provider);
    let project_path = project.get_project_path().unwrap();
    assert_eq!(project_path.to_str().unwrap(), "/workspace");
}

#[rstest]
#[case(
    Target::Arm64V8a,
    true,
    "/workspace/target/aarch64-linux-android/release"
)]
#[case(
    Target::Arm64V8a,
    false,
    "/workspace/target/aarch64-linux-android/debug"
)]
fn default_project_get_target_path(
    #[case] target: Target,
    #[case] release: bool,
    #[case] expected: String,
) {
    let provider = Box::new(MockManifest {});
    let project = DefaultProject::new(target, release, provider);
    let target_path = project.get_target_path().unwrap();
    assert_eq!(target_path.to_str().unwrap(), &expected);
}

struct MockManifest;
impl ManifestProvider for MockManifest {
    fn find_manifest_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/workspace/Cargo.toml"))
    }
}
