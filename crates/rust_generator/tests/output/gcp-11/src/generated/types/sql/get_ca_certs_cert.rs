#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCaCertsCert {
    /// The CA certificate used to connect to the SQL instance via SSL.
    #[builder(into)]
    #[serde(rename = "cert")]
    pub r#cert: Box<String>,
    /// The CN valid for the CA cert.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<String>,
    /// Creation time of the CA cert.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// Expiration time of the CA cert.
    #[builder(into)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: Box<String>,
    /// SHA1 fingerprint of the CA cert.
    #[builder(into)]
    #[serde(rename = "sha1Fingerprint")]
    pub r#sha_1_fingerprint: Box<String>,
}
