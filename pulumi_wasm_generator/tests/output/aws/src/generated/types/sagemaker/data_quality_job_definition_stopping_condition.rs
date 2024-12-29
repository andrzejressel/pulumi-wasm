#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionStoppingCondition {
    /// The maximum runtime allowed in seconds.
    #[builder(into, default)]
    #[serde(rename = "maxRuntimeInSeconds")]
    pub r#max_runtime_in_seconds: Box<Option<i32>>,
}
