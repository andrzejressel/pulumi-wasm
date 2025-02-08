#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogTableStorageDescriptorSchemaReference {
    /// Configuration block that contains schema identity fields. Either this or the `schema_version_id` has to be provided. See `schema_id` below.
    #[builder(into, default)]
    #[serde(rename = "schemaId")]
    pub r#schema_id: Box<Option<super::super::types::glue::CatalogTableStorageDescriptorSchemaReferenceSchemaId>>,
    /// Unique ID assigned to a version of the schema. Either this or the `schema_id` has to be provided.
    #[builder(into, default)]
    #[serde(rename = "schemaVersionId")]
    pub r#schema_version_id: Box<Option<String>>,
    /// Version number of the schema.
    #[builder(into)]
    #[serde(rename = "schemaVersionNumber")]
    pub r#schema_version_number: Box<i32>,
}
