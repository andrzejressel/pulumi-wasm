#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionProfileMysqlProfileSslConfig {
    /// PEM-encoded certificate of the CA that signed the source database
    /// server's certificate.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "caCertificate")]
    pub r#ca_certificate: Box<Option<String>>,
    /// (Output)
    /// Indicates whether the clientKey field is set.
    #[builder(into, default)]
    #[serde(rename = "caCertificateSet")]
    pub r#ca_certificate_set: Box<Option<bool>>,
    /// PEM-encoded certificate that will be used by the replica to
    /// authenticate against the source database server. If this field
    /// is used then the 'clientKey' and the 'caCertificate' fields are
    /// mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "clientCertificate")]
    pub r#client_certificate: Box<Option<String>>,
    /// (Output)
    /// Indicates whether the clientCertificate field is set.
    #[builder(into, default)]
    #[serde(rename = "clientCertificateSet")]
    pub r#client_certificate_set: Box<Option<bool>>,
    /// PEM-encoded private key associated with the Client Certificate.
    /// If this field is used then the 'client_certificate' and the
    /// 'ca_certificate' fields are mandatory.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    /// (Output)
    /// Indicates whether the clientKey field is set.
    #[builder(into, default)]
    #[serde(rename = "clientKeySet")]
    pub r#client_key_set: Box<Option<bool>>,
}
