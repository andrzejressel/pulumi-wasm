#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationInitialCapacity {
    /// The initial capacity configuration per worker.
    #[builder(into, default)]
    #[serde(rename = "initialCapacityConfig")]
    pub r#initial_capacity_config: Box<Option<super::super::types::emrserverless::ApplicationInitialCapacityInitialCapacityConfig>>,
    /// The worker type for an analytics framework. For Spark applications, the key can either be set to `Driver` or `Executor`. For Hive applications, it can be set to `HiveDriver` or `TezTask`.
    #[builder(into)]
    #[serde(rename = "initialCapacityType")]
    pub r#initial_capacity_type: Box<String>,
}
