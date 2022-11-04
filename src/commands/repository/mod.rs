use serde::Deserialize;

pub mod packageinfo;
pub mod packagelist;

#[derive(Deserialize, Debug)]
struct ResponseData {
    name: String,
    #[serde(alias = "prettyName")]
    pretty_name: String,
    version: String,
    #[serde(alias = "latestVersion")]
    latest_version: Option<String>,
    #[serde(alias = "packageName")]
    package_name: String,
    maintainer: String,
    description: String,
    url: String,
    #[serde(alias = "runtimeDependencies")]
    runtime_dependencies: Vec<String>,
    #[serde(alias = "buildDependencies")]
    build_dependencies: Vec<String>,
    #[serde(alias = "optionalDependencies")]
    optional_dependencies: Vec<String>,
    breaks: Vec<String>,
    gives: String,
    replace: Vec<String>,
    ppa: Vec<String>,
    #[serde(alias = "pacstallDependencies")]
    pacstall_dependencies: Vec<String>,
    repology: Vec<String>,
    #[serde(alias = "requiredBy")]
    required_by: Vec<String>,
    #[serde(alias = "updateStatus")]
    update_status: i8,
}

#[derive(Deserialize, Debug)]
struct PackagesResponse {
    total: u32,
    data: Vec<ResponseData>,
}

pub use packageinfo::packageinfo;
pub use packagelist::packagelist;
