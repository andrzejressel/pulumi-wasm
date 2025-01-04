#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterWorkloadIdentityConfig {
    /// The ID of the OIDC Identity Provider (IdP) associated to the Workload Identity Pool.
    #[builder(into, default)]
    #[serde(rename = "identityProvider")]
    pub r#identity_provider: Box<Option<String>>,
    /// The OIDC issuer URL for this cluster.
    #[builder(into, default)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: Box<Option<String>>,
    /// The Workload Identity Pool associated to the cluster.
    #[builder(into, default)]
    #[serde(rename = "workloadPool")]
    pub r#workload_pool: Box<Option<String>>,
}
