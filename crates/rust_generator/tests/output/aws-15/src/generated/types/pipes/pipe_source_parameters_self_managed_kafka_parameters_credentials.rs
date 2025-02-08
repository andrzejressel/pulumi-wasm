#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipeSourceParametersSelfManagedKafkaParametersCredentials {
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into, default)]
    #[serde(rename = "basicAuth")]
    pub r#basic_auth: Box<Option<String>>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into, default)]
    #[serde(rename = "clientCertificateTlsAuth")]
    pub r#client_certificate_tls_auth: Box<Option<String>>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into, default)]
    #[serde(rename = "saslScram256Auth")]
    pub r#sasl_scram_256_auth: Box<Option<String>>,
    /// The ARN of the Secrets Manager secret containing the credentials.
    #[builder(into, default)]
    #[serde(rename = "saslScram512Auth")]
    pub r#sasl_scram_512_auth: Box<Option<String>>,
}
