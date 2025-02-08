#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
