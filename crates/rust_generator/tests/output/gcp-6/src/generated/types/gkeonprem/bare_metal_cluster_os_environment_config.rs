#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterOsEnvironmentConfig {
    /// Whether the package repo should not be included when initializing
    /// bare metal machines.
    #[builder(into)]
    #[serde(rename = "packageRepoExcluded")]
    pub r#package_repo_excluded: Box<bool>,
}
