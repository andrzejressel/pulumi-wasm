#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeSourceParametersSelfManagedKafkaParameters {
    /// An array of server URLs. Maximum number of 2 items, each of maximum length 300.
    #[builder(into, default)]
    #[serde(rename = "additionalBootstrapServers")]
    pub r#additional_bootstrap_servers: Box<Option<Vec<String>>>,
    /// The maximum number of records to include in each batch. Maximum value of 10000.
    #[builder(into, default)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<Option<i32>>,
    /// The name of the destination queue to consume. Maximum value of 200.
    #[builder(into, default)]
    #[serde(rename = "consumerGroupId")]
    pub r#consumer_group_id: Box<Option<String>>,
    /// The credentials needed to access the resource. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "credentials")]
    pub r#credentials: Box<Option<super::super::types::pipes::PipeSourceParametersSelfManagedKafkaParametersCredentials>>,
    /// The maximum length of a time to wait for events. Maximum value of 300.
    #[builder(into, default)]
    #[serde(rename = "maximumBatchingWindowInSeconds")]
    pub r#maximum_batching_window_in_seconds: Box<Option<i32>>,
    /// The ARN of the Secrets Manager secret used for certification.
    #[builder(into, default)]
    #[serde(rename = "serverRootCaCertificate")]
    pub r#server_root_ca_certificate: Box<Option<String>>,
    /// The position in a stream from which to start reading. Valid values: TRIM_HORIZON, LATEST.
    #[builder(into, default)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: Box<Option<String>>,
    /// The name of the topic that the pipe will read from. Maximum length of 249.
    #[builder(into)]
    #[serde(rename = "topicName")]
    pub r#topic_name: Box<String>,
    /// This structure specifies the VPC subnets and security groups for the stream, and whether a public IP address is to be used. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "vpc")]
    pub r#vpc: Box<Option<super::super::types::pipes::PipeSourceParametersSelfManagedKafkaParametersVpc>>,
}
