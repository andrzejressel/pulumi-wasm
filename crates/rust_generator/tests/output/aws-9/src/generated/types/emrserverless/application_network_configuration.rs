#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationNetworkConfiguration {
    /// The array of security group Ids for customer VPC connectivity.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// The array of subnet Ids for customer VPC connectivity.
    #[builder(into, default)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Option<Vec<String>>>,
}
