#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeyKeyAttributesKeyModesOfUse {
    /// Whether an AWS Payment Cryptography key can be used to decrypt data.
    #[builder(into, default)]
    #[serde(rename = "decrypt")]
    pub r#decrypt: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used to derive new keys.
    #[builder(into, default)]
    #[serde(rename = "deriveKey")]
    pub r#derive_key: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used to encrypt data.
    #[builder(into, default)]
    #[serde(rename = "encrypt")]
    pub r#encrypt: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used to generate and verify other card and PIN verification keys.
    #[builder(into, default)]
    #[serde(rename = "generate")]
    pub r#generate: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key has no special restrictions other than the restrictions implied by KeyUsage.
    #[builder(into, default)]
    #[serde(rename = "noRestrictions")]
    pub r#no_restrictions: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used for signing.
    #[builder(into, default)]
    #[serde(rename = "sign")]
    pub r#sign: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used to unwrap other keys.
    #[builder(into, default)]
    #[serde(rename = "unwrap")]
    pub r#unwrap: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used to verify signatures.
    #[builder(into, default)]
    #[serde(rename = "verify")]
    pub r#verify: Box<Option<bool>>,
    /// Whether an AWS Payment Cryptography key can be used to wrap other keys.
    #[builder(into, default)]
    #[serde(rename = "wrap")]
    pub r#wrap: Box<Option<bool>>,
}
