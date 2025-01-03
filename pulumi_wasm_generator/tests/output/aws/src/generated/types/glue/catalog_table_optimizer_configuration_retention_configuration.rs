#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogTableOptimizerConfigurationRetentionConfiguration {
    /// The configuration for an Iceberg snapshot retention optimizer.
    #[builder(into, default)]
    #[serde(rename = "icebergConfiguration")]
    pub r#iceberg_configuration: Box<Option<super::super::types::glue::CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration>>,
}
