#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorAs2Config {
    /// Specifies weather AS2 file is compressed. The valud values are ZLIB and  DISABLED.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Box<String>,
    /// The algorithm that is used to encrypt the file. The valid values are AES128_CBC | AES192_CBC | AES256_CBC | NONE.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<String>,
    /// The unique identifier for the AS2 local profile.
    #[builder(into)]
    #[serde(rename = "localProfileId")]
    pub r#local_profile_id: Box<String>,
    /// Used for outbound requests to determine if a partner response for transfers is synchronous or asynchronous. The valid values are SYNC and NONE.
    #[builder(into)]
    #[serde(rename = "mdnResponse")]
    pub r#mdn_response: Box<String>,
    /// The signing algorithm for the Mdn response. The valid values are SHA256 | SHA384 | SHA512 | SHA1 | NONE | DEFAULT.
    #[builder(into, default)]
    #[serde(rename = "mdnSigningAlgorithm")]
    pub r#mdn_signing_algorithm: Box<Option<String>>,
    /// Used as the subject HTTP header attribute in AS2 messages that are being sent with the connector.
    #[builder(into, default)]
    #[serde(rename = "messageSubject")]
    pub r#message_subject: Box<Option<String>>,
    /// The unique identifier for the AS2 partner profile.
    #[builder(into)]
    #[serde(rename = "partnerProfileId")]
    pub r#partner_profile_id: Box<String>,
    /// The algorithm that is used to sign AS2 messages sent with the connector. The valid values are SHA256 | SHA384 | SHA512 | SHA1 | NONE .
    #[builder(into)]
    #[serde(rename = "signingAlgorithm")]
    pub r#signing_algorithm: Box<String>,
}
