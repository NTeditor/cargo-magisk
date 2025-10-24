use super::*;
use rstest::{fixture, rstest};

#[rstest]
#[case("/workspace")]
fn default_project_get_project_path(
    manifest_provider: Rc<dyn ManifestProvider>,
    #[case] expected: PathBuf,
) {
    let project = DefaultProject::new(Target::Arm64V8a, true, manifest_provider);
    let project_path = project.get_project_path().unwrap();
    assert_eq!(project_path, expected);
}

#[rstest]
fn default_project_get_target_path(
    #[values(Target::Arm64V8a, Target::ArmeabiV7a, Target::X86_64, Target::X86)] target: Target,
    #[values(true, false)] release: bool,
    manifest_provider: Rc<dyn ManifestProvider>,
) {
    let expected = get_expected_path(&target, release);
    let project = DefaultProject::new(target, release, manifest_provider);
    let target_path = project.get_target_path().unwrap();
    assert_eq!(target_path, expected);
}

fn get_expected_path(target: &Target, release: bool) -> PathBuf {
    let expected_str = match (target, release) {
        (&Target::Arm64V8a, true) => "/workspace/target/aarch64-linux-android/release",
        (&Target::Arm64V8a, false) => "/workspace/target/aarch64-linux-android/debug",
        (&Target::ArmeabiV7a, true) => "/workspace/target/armv7-linux-androideabi/release",
        (&Target::ArmeabiV7a, false) => "/workspace/target/armv7-linux-androideabi/debug",
        (&Target::X86_64, true) => "/workspace/target/x86_64-linux-android/release",
        (&Target::X86_64, false) => "/workspace/target/x86_64-linux-android/debug",
        (&Target::X86, true) => "/workspace/target/i686-linux-android/release",
        (&Target::X86, false) => "/workspace/target/i686-linux-android/debug",
    };
    PathBuf::from(expected_str)
}

#[fixture]
fn manifest_provider() -> Rc<dyn ManifestProvider> {
    Rc::new(MockManifest {})
}

#[derive(Debug)]
struct MockManifest;
impl ManifestProvider for MockManifest {
    fn find_manifest_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/workspace/Cargo.toml"))
    }
}
