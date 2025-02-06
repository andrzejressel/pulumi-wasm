#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCatalogTableStorageDescriptorSchemaReference {
    /// Configuration block that contains schema identity fields. See `schema_id` below.
    #[builder(into)]
    #[serde(rename = "schemaIds")]
    pub r#schema_ids: Box<Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSchemaReferenceSchemaId>>,
    /// Unique ID assigned to a version of the schema.
    #[builder(into)]
    #[serde(rename = "schemaVersionId")]
    pub r#schema_version_id: Box<String>,
    /// Version number of the schema.
    #[builder(into)]
    #[serde(rename = "schemaVersionNumber")]
    pub r#schema_version_number: Box<i32>,
}
