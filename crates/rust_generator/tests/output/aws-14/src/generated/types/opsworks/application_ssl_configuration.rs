#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationSslConfiguration {
    /// The contents of the certificate's domain.crt file.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<String>,
    /// Can be used to specify an intermediate certificate authority key or client authentication.
    #[builder(into, default)]
    #[serde(rename = "chain")]
    pub r#chain: Box<Option<String>>,
    /// The private key; the contents of the certificate's domain.key file.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<String>,
}
