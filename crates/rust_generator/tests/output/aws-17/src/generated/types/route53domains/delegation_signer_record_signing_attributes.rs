#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DelegationSignerRecordSigningAttributes {
    /// Algorithm which was used to generate the digest from the public key.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<i32>,
    /// Defines the type of key. It can be either a KSK (key-signing-key, value `257`) or ZSK (zone-signing-key, value `256`).
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Box<i32>,
    /// The base64-encoded public key part of the key pair that is passed to the registry.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
