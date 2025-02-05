#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogTableOpenTableFormatInput {
    /// Configuration block for iceberg table config. See `iceberg_input` below.
    #[builder(into)]
    #[serde(rename = "icebergInput")]
    pub r#iceberg_input: Box<super::super::types::glue::CatalogTableOpenTableFormatInputIcebergInput>,
}
