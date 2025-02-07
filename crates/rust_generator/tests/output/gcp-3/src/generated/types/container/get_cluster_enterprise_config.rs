#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterEnterpriseConfig {
    /// Indicates the effective cluster tier. Available options include STANDARD and ENTERPRISE.
    #[builder(into)]
    #[serde(rename = "clusterTier")]
    pub r#cluster_tier: Box<String>,
    /// Indicates the desired cluster tier. Available options include STANDARD and ENTERPRISE.
    #[builder(into)]
    #[serde(rename = "desiredTier")]
    pub r#desired_tier: Box<String>,
}
