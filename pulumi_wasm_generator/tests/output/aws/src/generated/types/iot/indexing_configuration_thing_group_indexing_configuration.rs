#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexingConfigurationThingGroupIndexingConfiguration {
    /// A list of thing group fields to index. This list cannot contain any managed fields. See below.
    #[builder(into, default)]
    #[serde(rename = "customFields")]
    pub r#custom_fields: Box<Option<Vec<super::super::types::iot::IndexingConfigurationThingGroupIndexingConfigurationCustomField>>>,
    /// Contains fields that are indexed and whose types are already known by the Fleet Indexing service. See below.
    #[builder(into, default)]
    #[serde(rename = "managedFields")]
    pub r#managed_fields: Box<Option<Vec<super::super::types::iot::IndexingConfigurationThingGroupIndexingConfigurationManagedField>>>,
    /// Thing group indexing mode. Valid values: `OFF`, `ON`.
    #[builder(into)]
    #[serde(rename = "thingGroupIndexingMode")]
    pub r#thing_group_indexing_mode: Box<String>,
}