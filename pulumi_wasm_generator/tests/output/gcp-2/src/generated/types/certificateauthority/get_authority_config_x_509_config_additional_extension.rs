#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthorityConfigX509ConfigAdditionalExtension {
    /// Indicates whether or not this extension is critical (i.e., if the client does not know how to
    /// handle this extension, the client should consider this to be an error).
    #[builder(into)]
    #[serde(rename = "critical")]
    pub r#critical: Box<bool>,
    /// Describes values that are relevant in a CA certificate.
    #[builder(into)]
    #[serde(rename = "objectIds")]
    pub r#object_ids: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigAdditionalExtensionObjectId>>,
    /// The value of this X.509 extension. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
