use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Manifest {
    pub package: Package,
}

#[derive(Deserialize)]
pub(super) struct Package {
    pub version: String,
    pub metadata: Metadata,
}

#[derive(Deserialize)]
pub(super) struct Metadata {
    pub magisk: Magisk,
}

#[derive(Deserialize)]
pub(super) struct Magisk {
    pub id: String,
    pub name: String,
    pub author: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize)]
pub(super) struct Asset {
    pub source: String,
    pub dest: String,
}
