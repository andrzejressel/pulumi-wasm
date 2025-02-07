#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfigCryptoKeyKmsWrapped {
    /// The resource name of the KMS CryptoKey to use for unwrapping.
    #[builder(into)]
    #[serde(rename = "cryptoKeyName")]
    pub r#crypto_key_name: Box<String>,
    /// The wrapped data crypto key.
    /// A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "wrappedKey")]
    pub r#wrapped_key: Box<String>,
}
