#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMasterAuthorizedNetworksConfig {
    /// External networks that can access the
    /// Kubernetes cluster master through HTTPS.
    #[builder(into, default)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Box<Option<Vec<super::super::types::container::ClusterMasterAuthorizedNetworksConfigCidrBlock>>>,
    /// Whether Kubernetes master is
    /// accessible via Google Compute Engine Public IPs.
    #[builder(into, default)]
    #[serde(rename = "gcpPublicCidrsAccessEnabled")]
    pub r#gcp_public_cidrs_access_enabled: Box<Option<bool>>,
    /// Whether authorized networks is enforced on the private endpoint or not. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "privateEndpointEnforcementEnabled")]
    pub r#private_endpoint_enforcement_enabled: Box<Option<bool>>,
}
