#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyDefinitionTemplateLinkedPrincipal {
    /// The entity ID of the principal.
    #[builder(into)]
    #[serde(rename = "entityId")]
    pub r#entity_id: Box<String>,
    /// The entity type of the principal.
    #[builder(into)]
    #[serde(rename = "entityType")]
    pub r#entity_type: Box<String>,
}