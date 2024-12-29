#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FieldLevelEncryptionProfileEncryptionEntitiesItem {
    /// Object that contains an attribute `items` that contains the list of field patterns in a field-level encryption content type profile specify the fields that you want to be encrypted.
    #[builder(into)]
    #[serde(rename = "fieldPatterns")]
    pub r#field_patterns: Box<super::super::types::cloudfront::FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatterns>,
    /// The provider associated with the public key being used for encryption.
    #[builder(into)]
    #[serde(rename = "providerId")]
    pub r#provider_id: Box<String>,
    /// The public key associated with a set of field-level encryption patterns, to be used when encrypting the fields that match the patterns.
    #[builder(into)]
    #[serde(rename = "publicKeyId")]
    pub r#public_key_id: Box<String>,
}
