#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TargetGroupConfig {
    /// The health check configuration.
    #[builder(into, default)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Box<Option<super::super::types::vpclattice::TargetGroupConfigHealthCheck>>,
    /// The type of IP address used for the target group. Valid values: `IPV4` | `IPV6`.
    #[builder(into, default)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Box<Option<String>>,
    /// The version of the event structure that the Lambda function receives. Supported only if `type` is `LAMBDA`. Valid Values are `V1` | `V2`.
    #[builder(into, default)]
    #[serde(rename = "lambdaEventStructureVersion")]
    pub r#lambda_event_structure_version: Box<Option<String>>,
    /// The port on which the targets are listening.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The protocol to use for routing traffic to the targets. Valid Values are `HTTP` | `HTTPS`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// The protocol version. Valid Values are `HTTP1` | `HTTP2` | `GRPC`. Default value is `HTTP1`.
    #[builder(into, default)]
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: Box<Option<String>>,
    /// The ID of the VPC.
    #[builder(into, default)]
    #[serde(rename = "vpcIdentifier")]
    pub r#vpc_identifier: Box<Option<String>>,
}
