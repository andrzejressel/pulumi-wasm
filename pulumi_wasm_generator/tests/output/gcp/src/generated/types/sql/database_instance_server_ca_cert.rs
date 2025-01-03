#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceServerCaCert {
    /// The CA Certificate used to connect to the SQL Instance via SSL.
    #[builder(into, default)]
    #[serde(rename = "cert")]
    pub r#cert: Box<Option<String>>,
    /// The CN valid for the CA Cert.
    #[builder(into, default)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// Creation time of the CA Cert.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Expiration time of the CA Cert.
    #[builder(into, default)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: Box<Option<String>>,
    /// SHA Fingerprint of the CA Cert.
    #[builder(into, default)]
    #[serde(rename = "sha1Fingerprint")]
    pub r#sha_1_fingerprint: Box<Option<String>>,
}
