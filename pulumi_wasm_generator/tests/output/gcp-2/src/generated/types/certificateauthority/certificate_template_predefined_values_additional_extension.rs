#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateTemplatePredefinedValuesAdditionalExtension {
    /// Optional. Indicates whether or not this extension is critical (i.e., if the client does not know how to handle this extension, the client should consider this to be an error).
    #[builder(into, default)]
    #[serde(rename = "critical")]
    pub r#critical: Box<Option<bool>>,
    /// Required. The OID for this X.509 extension.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesAdditionalExtensionObjectId>,
    /// Required. The value of this X.509 extension.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
