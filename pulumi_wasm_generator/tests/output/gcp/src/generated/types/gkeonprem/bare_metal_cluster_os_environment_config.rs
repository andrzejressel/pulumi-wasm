#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterOsEnvironmentConfig {
    /// Whether the package repo should not be included when initializing
    /// bare metal machines.
    #[builder(into)]
    #[serde(rename = "packageRepoExcluded")]
    pub r#package_repo_excluded: Box<bool>,
}
