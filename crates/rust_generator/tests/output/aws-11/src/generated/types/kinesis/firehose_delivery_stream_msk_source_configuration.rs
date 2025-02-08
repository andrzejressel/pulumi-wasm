#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirehoseDeliveryStreamMskSourceConfiguration {
    /// The authentication configuration of the Amazon MSK cluster. See `authentication_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "authenticationConfiguration")]
    pub r#authentication_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamMskSourceConfigurationAuthenticationConfiguration>,
    /// The ARN of the Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "mskClusterArn")]
    pub r#msk_cluster_arn: Box<String>,
    /// The topic name within the Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "topicName")]
    pub r#topic_name: Box<String>,
}
