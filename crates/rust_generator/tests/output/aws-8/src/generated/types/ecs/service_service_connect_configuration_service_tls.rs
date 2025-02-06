#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceServiceConnectConfigurationServiceTls {
    /// Details of the certificate authority which will issue the certificate.
    #[builder(into)]
    #[serde(rename = "issuerCertAuthority")]
    pub r#issuer_cert_authority: Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTlsIssuerCertAuthority>,
    /// KMS key used to encrypt the private key in Secrets Manager.
    #[builder(into, default)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<Option<String>>,
    /// ARN of the IAM Role that's associated with the Service Connect TLS.
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
}
