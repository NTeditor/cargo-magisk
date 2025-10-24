use std::path::Path;

use super::*;
use rstest::rstest;

#[rstest]
#[case(Target::ARM64_V8A_STR, Target::Arm64V8a)]
#[case(Target::ARMEABI_V7A_STR, Target::ArmeabiV7a)]
#[case(Target::X86_64_STR, Target::X86_64)]
#[case(Target::X86_STR, Target::X86)]
fn target_enum_from(#[case] target_string: &str, #[case] expected: Target) {
    let target = Target::try_from(target_string).unwrap();
    assert_eq!(target, expected);
}

#[rstest]
#[case(Target::Arm64V8a, Target::ARM64_V8A_STR)]
#[case(Target::ArmeabiV7a, Target::ARMEABI_V7A_STR)]
#[case(Target::X86_64, Target::X86_64_STR)]
#[case(Target::X86, Target::X86_STR)]
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
