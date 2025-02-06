#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainNameDomainNameConfiguration {
    /// ARN of an AWS-managed certificate that will be used by the endpoint for the domain name. AWS Certificate Manager is the only supported source. Use the `aws.acm.Certificate` resource to configure an ACM certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: Box<String>,
    /// Endpoint type. Valid values: `REGIONAL`.
    #[builder(into)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: Box<String>,
    /// Amazon Route 53 Hosted Zone ID of the endpoint.
    #[builder(into, default)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Box<Option<String>>,
    /// ARN of the AWS-issued certificate used to validate custom domain ownership (when `certificate_arn` is issued via an ACM Private CA or `mutual_tls_authentication` is configured with an ACM-imported certificate.)
    #[builder(into, default)]
    #[serde(rename = "ownershipVerificationCertificateArn")]
    pub r#ownership_verification_certificate_arn: Box<Option<String>>,
    /// Transport Layer Security (TLS) version of the [security policy](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-custom-domain-tls-version.html) for the domain name. Valid values: `TLS_1_2`.
    #[builder(into)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Box<String>,
    /// Target domain name.
    #[builder(into, default)]
    #[serde(rename = "targetDomainName")]
    pub r#target_domain_name: Box<Option<String>>,
}
