use super::*;
use rstest::rstest;

#[rstest]
#[case("1.0.1", [0, 1, 0, 0, 0, 1, 9, 0 , 0])]
#[case("1.0.1-alpha.1", [0, 1, 0, 0, 0, 1, 1, 0, 1])]
#[case("1.0.0", [0, 1, 0, 0, 0, 0, 9, 0, 0])]
#[case("1.0.0-alpha.1", [0, 1, 0, 0, 0, 0, 1, 0, 1])]
#[case("1.0.0-beta.1", [0, 1, 0, 0, 0, 0, 2, 0, 1])]
#[case("1.0.0-rc.1", [0, 1, 0, 0, 0, 0, 3, 0, 1])]
#[case("2.5.10", [0, 2, 0, 5, 1, 0, 9, 0, 0])]
#[case("2.5.10-beta.5", [0, 2, 0, 5, 1, 0, 2, 0, 5])]
#[case("99.99.99", [9, 9, 9, 9, 9, 9, 9, 0, 0])]
#[case("99.99.99-rc.99", [9, 9, 9, 9, 9, 9, 3, 9, 9])]
fn version_code_from_str(#[case] version_str: &str, #[case] expected: [u8; 9]) {
    let version_code = VersionCode::try_from(version_str).unwrap();
    assert_eq!(version_code.0, expected);
}

#[rstest]
#[case("999.999.999")]
#[case("1.0")]
#[case("1.0.0.1")]
#[case("1.0.0-invalid.1")]
#[case("a.b.c")]
#[case("1.0.0-alpha.100")]
#[case("100.0.0")]
#[case("")]
#[case("1.0.0-stable.1")]
fn version_code_from_str_err(#[case] version_str: &str) {
    let result = VersionCode::try_from(version_str);
    assert!(
        result.is_err(),
        "result.is_err() is not true. version_str: '{}'",
        version_str,
    )
}
