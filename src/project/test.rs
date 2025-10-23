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

struct MockProjectProvider;
