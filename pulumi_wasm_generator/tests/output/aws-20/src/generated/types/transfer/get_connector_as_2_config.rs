#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConnectorAs2Config {
    /// Basic authentication for AS2 connector API. Returns a null value if not set.
    #[builder(into)]
    #[serde(rename = "basicAuthSecretId")]
    pub r#basic_auth_secret_id: Box<String>,
    /// Specifies whether AS2 file is compressed. Will be ZLIB or DISABLED
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Box<String>,
    /// Algorithm used to encrypt file. Will be AES128_CBC or AES192_CBC or AES256_CBC or DES_EDE3_CBC or NONE.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<String>,
    /// Unique identifier for AS2 local profile.
    #[builder(into)]
    #[serde(rename = "localProfileId")]
    pub r#local_profile_id: Box<String>,
    /// Used for outbound requests to tell if response is asynchronous or not. Will be either SYNC or NONE.
    #[builder(into)]
    #[serde(rename = "mdnResponse")]
    pub r#mdn_response: Box<String>,
    /// Signing algorithm for MDN response. Will be SHA256 or SHA384 or SHA512 or SHA1 or NONE or DEFAULT.
    #[builder(into)]
    #[serde(rename = "mdnSigningAlgorithm")]
    pub r#mdn_signing_algorithm: Box<String>,
    /// Subject HTTP header attribute in outbound AS2 messages to the connector.
    #[builder(into)]
    #[serde(rename = "messageSubject")]
    pub r#message_subject: Box<String>,
    /// Unique identifier used by connector for partner profile.
    #[builder(into)]
    #[serde(rename = "partnerProfileId")]
    pub r#partner_profile_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "singingAlgorithm")]
    pub r#singing_algorithm: Box<String>,
}
