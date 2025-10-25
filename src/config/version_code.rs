#[cfg(test)]
mod test;

use anyhow::{Context, bail};
use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
pub struct VersionCode([u8; 9]);

impl VersionCode {
    const MAJOR_POS: usize = 0;
    const MINOR_POS: usize = 2;
    const PATCH_POS: usize = 4;
    const PRE_TYPE_POS: usize = 6;
    const PRE_CODE_POS: usize = 7;

    fn set_two_digits(value: u8, arr: &mut [u8; 9], start_arr: usize) {
        if value >= 10 {
            arr[start_arr] = value / 10;
            arr[start_arr + 1] = value % 10;
        } else {
            arr[start_arr] = 0;
            arr[start_arr + 1] = value;
        }
    }
}

impl TryFrom<&str> for VersionCode {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"^(?P<major>\d{1,2})\.(?P<minor>\d{1,2})\.(?P<patch>\d{1,2})(?:-(?P<pre_type>alpha|beta|rc)(?:\.(?P<pre_code>\d{1,2}))?)?$",
        )?;

        let caps = re.captures(value).context("Invalid version format")?;
        let major: u8 = caps["major"].parse().context("Invalid major version")?;
        let minor: u8 = caps["minor"].parse().context("Invalid minor version")?;
        let patch: u8 = caps["patch"].parse().context("Invalid patch version")?;
        let mut version_code: [u8; 9] = [0; 9];
        Self::set_two_digits(major, &mut version_code, Self::MAJOR_POS);
        Self::set_two_digits(minor, &mut version_code, Self::MINOR_POS);
        Self::set_two_digits(patch, &mut version_code, Self::PATCH_POS);

        if let Some(pre_type_str) = caps.name("pre_type") {
            let pre_type =
                ReleaseType::try_from(pre_type_str.as_str()).context("Invalid pre-type")?;
            version_code[Self::PRE_TYPE_POS] = pre_type as u8;

            if let Some(pre_code_str) = caps.name("pre_code") {
                let pre_code: u8 = pre_code_str.as_str().parse().context("Invalid pre-type")?;
                Self::set_two_digits(pre_code, &mut version_code, Self::PRE_CODE_POS);
            }
        } else {
            let pre_type = ReleaseType::Stable;
            version_code[6] = pre_type as u8;
        }

        Ok(Self(version_code))
    }
}

impl Display for VersionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(9);
        for &digit in self.0.iter() {
            s.push((digit + b'0') as char);
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
#[repr(u8)]
enum ReleaseType {
    Alpha = 1,
    Beta = 2,
    Rc = 3,
    Stable = 9,
}

impl TryFrom<&str> for ReleaseType {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "alpha" => Ok(ReleaseType::Alpha),
            "beta" => Ok(ReleaseType::Beta),
            "rc" => Ok(ReleaseType::Rc),
            _ => bail!("Invalid build type: {}", name),
        }
    }
}
