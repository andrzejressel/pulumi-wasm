#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionProfileOracleSsl {
    /// Required. Input only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.
    /// The replica will use this certificate to verify it's connecting to the right host.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "caCertificate")]
    pub r#ca_certificate: Box<String>,
    /// Input only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.
    /// If this field is used then the 'clientKey' field is mandatory
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<Option<String>>,
    /// Input only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.
    /// If this field is used then the 'clientCertificate' field is mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    /// (Output)
    /// The current connection profile state.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
