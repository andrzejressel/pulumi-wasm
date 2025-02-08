#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeyRingImportJobPublicKey {
    /// (Output)
    /// The public key, encoded in PEM format. For more information, see the RFC 7468 sections
    /// for General Considerations and Textual Encoding of Subject Public Key Info.
    #[builder(into, default)]
    #[serde(rename = "pem")]
    pub r#pem: Box<Option<String>>,
}
