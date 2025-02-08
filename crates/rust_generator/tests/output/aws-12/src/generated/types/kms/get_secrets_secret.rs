#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecretsSecret {
    /// An optional mapping that makes up the Encryption Context for the secret.
    #[builder(into, default)]
    #[serde(rename = "context")]
    pub r#context: Box<Option<std::collections::HashMap<String, String>>>,
    /// The encryption algorithm that will be used to decrypt the ciphertext. This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key. Valid Values: SYMMETRIC_DEFAULT | RSAES_OAEP_SHA_1 | RSAES_OAEP_SHA_256 | SM2PKE
    #[builder(into, default)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<Option<String>>,
    /// An optional list of Grant Tokens for the secret.
    #[builder(into, default)]
    #[serde(rename = "grantTokens")]
    pub r#grant_tokens: Box<Option<Vec<String>>>,
    /// Specifies the KMS key that AWS KMS uses to decrypt the ciphertext. This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key.
    /// 
    /// For more information on `context` and `grant_tokens` see the [KMS
    /// Concepts](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html)
    #[builder(into, default)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<Option<String>>,
    /// Name to export this secret under in the attributes.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Base64 encoded payload, as returned from a KMS encrypt operation.
    #[builder(into)]
    #[serde(rename = "payload")]
    pub r#payload: Box<String>,
}
