#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainSamlOptionsSamlOptionsIdp {
    /// The unique Entity ID of the application in SAML Identity Provider.
    #[builder(into)]
    #[serde(rename = "entityId")]
    pub r#entity_id: Box<String>,
    /// The Metadata of the SAML application in xml format.
    #[builder(into)]
    #[serde(rename = "metadataContent")]
    pub r#metadata_content: Box<String>,
}