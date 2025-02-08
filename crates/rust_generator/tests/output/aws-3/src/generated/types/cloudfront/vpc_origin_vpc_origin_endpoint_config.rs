#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VpcOriginVpcOriginEndpointConfig {
    /// The VPC origin ARN.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The HTTP port for the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: Box<i32>,
    /// The HTTPS port for the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: Box<i32>,
    /// The name of the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The origin protocol policy for the CloudFront VPC origin endpoint configuration.
    #[builder(into)]
    #[serde(rename = "originProtocolPolicy")]
    pub r#origin_protocol_policy: Box<String>,
    /// A complex type that contains information about the SSL/TLS protocols that CloudFront can use when establishing an HTTPS connection with your origin.
    #[builder(into, default)]
    #[serde(rename = "originSslProtocols")]
    pub r#origin_ssl_protocols: Box<Option<super::super::types::cloudfront::VpcOriginVpcOriginEndpointConfigOriginSslProtocols>>,
}
