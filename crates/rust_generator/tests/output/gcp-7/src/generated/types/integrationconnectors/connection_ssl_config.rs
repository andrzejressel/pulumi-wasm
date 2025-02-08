#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionSslConfig {
    /// Additional SSL related field values.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Box<Option<Vec<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariable>>>,
    /// Type of Client Cert (PEM/JKS/.. etc.)
    /// Possible values are: `PEM`.
    #[builder(into, default)]
    #[serde(rename = "clientCertType")]
    pub r#client_cert_type: Box<Option<String>>,
    /// Client Certificate
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigClientCertificate>>,
    /// Client Private Key
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "clientPrivateKey")]
    pub r#client_private_key: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigClientPrivateKey>>,
    /// Secret containing the passphrase protecting the Client Private Key
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "clientPrivateKeyPass")]
    pub r#client_private_key_pass: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigClientPrivateKeyPass>>,
    /// Private Server Certificate. Needs to be specified if trust model is PRIVATE.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "privateServerCertificate")]
    pub r#private_server_certificate: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigPrivateServerCertificate>>,
    /// Type of Server Cert (PEM/JKS/.. etc.)
    /// Possible values are: `PEM`.
    #[builder(into, default)]
    #[serde(rename = "serverCertType")]
    pub r#server_cert_type: Box<Option<String>>,
    /// Enum for Trust Model
    /// Possible values are: `PUBLIC`, `PRIVATE`, `INSECURE`.
    #[builder(into, default)]
    #[serde(rename = "trustModel")]
    pub r#trust_model: Box<Option<String>>,
    /// Enum for controlling the SSL Type (TLS/MTLS)
    /// Possible values are: `TLS`, `MTLS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Bool for enabling SSL
    #[builder(into, default)]
    #[serde(rename = "useSsl")]
    pub r#use_ssl: Box<Option<bool>>,
}
