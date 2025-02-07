#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKeyMultiRegionConfigurationReplicaKey {
    /// The key ARN of a primary or replica key of a multi-Region key.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The AWS Region of a primary or replica key in a multi-Region key.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
