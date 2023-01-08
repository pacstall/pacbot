use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub name: String,
    #[serde(alias = "prettyName")]
    pub pretty_name: String,
    pub version: String,
    #[serde(alias = "latestVersion")]
    pub latest_version: Option<String>,
    #[serde(alias = "packageName")]
    pub package_name: String,
    pub maintainer: String,
    pub description: String,
    pub url: String,
    #[serde(alias = "runtimeDependencies")]
    pub runtime_dependencies: Vec<String>,
    #[serde(alias = "buildDependencies")]
    pub build_dependencies: Vec<String>,
    #[serde(alias = "optionalDependencies")]
    pub optional_dependencies: Vec<String>,
    pub breaks: Vec<String>,
    pub gives: String,
    pub replace: Vec<String>,
    pub ppa: Vec<String>,
    #[serde(alias = "pacstallDependencies")]
    pub pacstall_dependencies: Vec<String>,
    pub repology: Vec<String>,
    #[serde(alias = "requiredBy")]
    pub required_by: Vec<String>,
    #[serde(alias = "updateStatus")]
    pub update_status: i8,
}

#[derive(Deserialize, Debug)]
pub struct PackagesResponse {
    pub total: u32,
    #[serde(alias = "lastPage")]
    pub last_page: u32,
    pub data: Vec<ResponseData>,
}
