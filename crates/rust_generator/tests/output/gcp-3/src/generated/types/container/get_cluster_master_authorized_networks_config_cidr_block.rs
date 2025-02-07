#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterMasterAuthorizedNetworksConfigCidrBlock {
    /// External network that can access Kubernetes master through HTTPS. Must be specified in CIDR notation.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<String>,
    /// Field for users to identify CIDR blocks.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
}
