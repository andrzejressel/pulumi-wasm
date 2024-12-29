#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexingConfigurationThingIndexingConfiguration {
    /// Contains custom field names and their data type. See below.
    #[builder(into, default)]
    #[serde(rename = "customFields")]
    pub r#custom_fields: Box<Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationCustomField>>>,
    /// Device Defender indexing mode. Valid values: `VIOLATIONS`, `OFF`. Default: `OFF`.
    #[builder(into, default)]
    #[serde(rename = "deviceDefenderIndexingMode")]
    pub r#device_defender_indexing_mode: Box<Option<String>>,
    /// Required if `named_shadow_indexing_mode` is `ON`. Enables to add named shadows filtered by `filter` to fleet indexing configuration.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationFilter>>,
    /// Contains fields that are indexed and whose types are already known by the Fleet Indexing service. See below.
    #[builder(into, default)]
    #[serde(rename = "managedFields")]
    pub r#managed_fields: Box<Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationManagedField>>>,
    /// [Named shadow](https://docs.aws.amazon.com/iot/latest/developerguide/iot-device-shadows.html) indexing mode. Valid values: `ON`, `OFF`. Default: `OFF`.
    #[builder(into, default)]
    #[serde(rename = "namedShadowIndexingMode")]
    pub r#named_shadow_indexing_mode: Box<Option<String>>,
    /// Thing connectivity indexing mode. Valid values: `STATUS`, `OFF`. Default: `OFF`.
    #[builder(into, default)]
    #[serde(rename = "thingConnectivityIndexingMode")]
    pub r#thing_connectivity_indexing_mode: Box<Option<String>>,
    /// Thing indexing mode. Valid values: `REGISTRY`, `REGISTRY_AND_SHADOW`, `OFF`.
    #[builder(into)]
    #[serde(rename = "thingIndexingMode")]
    pub r#thing_indexing_mode: Box<String>,
}
