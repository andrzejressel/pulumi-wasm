#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptions {
    /// Defines the metadata for a schema property.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "propertyDefinitions")]
    pub r#property_definitions: Box<Vec<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinition>>,
}
