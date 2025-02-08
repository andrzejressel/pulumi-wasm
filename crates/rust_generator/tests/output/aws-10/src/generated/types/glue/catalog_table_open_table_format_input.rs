#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CatalogTableOpenTableFormatInput {
    /// Configuration block for iceberg table config. See `iceberg_input` below.
    #[builder(into)]
    #[serde(rename = "icebergInput")]
    pub r#iceberg_input: Box<super::super::types::glue::CatalogTableOpenTableFormatInputIcebergInput>,
}
