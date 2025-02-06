#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterMasterAuthorizedNetworksConfig {
    /// External networks that can access the Kubernetes cluster master through HTTPS.
    #[builder(into)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Box<Vec<super::super::types::container::GetClusterMasterAuthorizedNetworksConfigCidrBlock>>,
    /// Whether Kubernetes master is accessible via Google Compute Engine Public IPs.
    #[builder(into)]
    #[serde(rename = "gcpPublicCidrsAccessEnabled")]
    pub r#gcp_public_cidrs_access_enabled: Box<bool>,
    /// Whether authorized networks is enforced on the private endpoint or not. Defaults to false.
    #[builder(into)]
    #[serde(rename = "privateEndpointEnforcementEnabled")]
    pub r#private_endpoint_enforcement_enabled: Box<bool>,
}
