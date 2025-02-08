#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterConfigurationInfo {
    /// Amazon Resource Name (ARN) of the MSK Configuration to use in the cluster.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Revision of the MSK Configuration to use in the cluster.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<i32>,
}
