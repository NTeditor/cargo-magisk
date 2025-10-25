use super::*;
use crate::project::Target;
use anyhow::Result;
use rstest::{fixture, rstest};

#[rstest]
fn asset_parse_source(
    #[values("target/cargo-magisk", "assets/customize.sh")] source: String,
    project_provider: Rc<dyn ProjectProvider>,
) {
    let expected = get_expected_parse_source(&source);
    let source_path = Asset::parse_source(source, &project_provider).unwrap();
    assert_eq!(source_path, expected);
}

fn get_expected_parse_source(source: &str) -> PathBuf {
    let expected_str = match source {
        "target/cargo-magisk" => "/workspace/target/arch/build_type/cargo-magisk",
        "assets/customize.sh" => "/workspace/assets/customize.sh",
        _ => {
            panic!("Invalid source")
        }
    };
    PathBuf::from(expected_str)
}

#[rstest]
fn asset_parse_dest(
    #[values("system/bin/cargo-magisk", "customize.sh")] dest: String,
    project_provider: Rc<dyn ProjectProvider>,
) {
    let expected = get_expected_parse_dest(&dest);
    let dest_path = Asset::parse_dest(dest, &project_provider).unwrap();
    assert_eq!(dest_path, expected);
}

fn get_expected_parse_dest(dest: &str) -> PathBuf {
    let expected_str = match dest {
        "system/bin/cargo-magisk" => {
            "/workspace/target/arch/build_type/magisk/system/bin/cargo-magisk"
        }
        "customize.sh" => "/workspace/target/arch/build_type/magisk/customize.sh",
        _ => {
            panic!("Invalid source")
        }
    };
    PathBuf::from(expected_str)
}

#[rstest]
fn asset_parse_source_err(
    #[values(
        "",
        "target/../asset/customize.sh",
        "target/.../asset/customize.sh",
        "./target/cargo-magisk",
        "./../cargo-magisk",
        ".",
        "..",
        "..."
    )]
    source: String,
    project_provider: Rc<dyn ProjectProvider>,
) {
    let source_path = Asset::parse_source(source.clone(), &project_provider);
    assert!(
        source_path.is_err(),
        "source_path.is_err() not true. source: '{}'",
        source
    );
}

#[rstest]
fn asset_parse_dest_err(
    #[values(
        "",
        "../customize.sh",
        "./system/./bin/cargo-magisk",
        "./system/.../bin/cargo-magisk",
        "./../cargo-magisk",
        ".",
        "..",
        "..."
    )]
    dest: String,
    project_provider: Rc<dyn ProjectProvider>,
) {
    let dest_path = Asset::parse_dest(dest.clone(), &project_provider);
    assert!(
        dest_path.is_err(),
        "dest_path.is_err() not true. dest: '{}'",
        dest
    );
}

#[rstest]
fn module_prop_validate(
    #[values("a_module", "a.module", "module-101")] id: String,
    #[values("valid name")] name: String,
    #[values("1.0.0")] version: String,
    #[values("valid author")] author: String,
) {
    let result = ModuleProp::validate(&id, &name, &version, &author);
    assert!(
        result.is_ok(),
        "result.is_ok() not true. id: '{}', name: '{}', version: '{}', author: '{}'",
        id,
        name,
        version,
        author,
    );
}

#[rstest]
fn module_prop_validate_err(
    #[values("", "1_module", "a module", "-a-module")] id: String,
    #[values("")] name: String,
    #[values("")] version: String,
    #[values("")] author: String,
) {
    let result = ModuleProp::validate(&id, &name, &version, &author);
    assert!(
        result.is_err(),
        "result.is_err() not true. id: '{}', name: '{}', version: '{}', author: '{}'",
        id,
        name,
        version,
        author,
    );
}

#[fixture]
fn project_provider() -> Rc<dyn ProjectProvider> {
    Rc::new(MockProject {})
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
