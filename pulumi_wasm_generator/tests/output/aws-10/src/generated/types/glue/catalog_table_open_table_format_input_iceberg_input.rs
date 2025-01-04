#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogTableOpenTableFormatInputIcebergInput {
    /// A required metadata operation. Can only be set to CREATE.
    #[builder(into)]
    #[serde(rename = "metadataOperation")]
    pub r#metadata_operation: Box<String>,
    /// The table version for the Iceberg table. Defaults to 2.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
