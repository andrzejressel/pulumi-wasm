#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryVirtualRepositoryConfig {
    /// Policies that configure the upstream artifacts distributed by the Virtual
    /// Repository. Upstream policies cannot be set on a standard repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "upstreamPolicies")]
    pub r#upstream_policies: Box<Option<Vec<super::super::types::artifactregistry::RepositoryVirtualRepositoryConfigUpstreamPolicy>>>,
}
