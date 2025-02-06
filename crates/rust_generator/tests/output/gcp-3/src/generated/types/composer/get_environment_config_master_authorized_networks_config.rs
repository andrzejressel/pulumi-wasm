#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentConfigMasterAuthorizedNetworksConfig {
    /// cidr_blocks define up to 50 external networks that could access Kubernetes master through HTTPS.
    #[builder(into)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Box<Vec<super::super::types::composer::GetEnvironmentConfigMasterAuthorizedNetworksConfigCidrBlock>>,
    /// Whether or not master authorized networks is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
