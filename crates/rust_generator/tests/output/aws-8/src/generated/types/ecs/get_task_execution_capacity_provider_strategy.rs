#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTaskExecutionCapacityProviderStrategy {
    /// The number of tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "base")]
    pub r#base: Box<Option<i32>>,
    /// Name of the capacity provider.
    #[builder(into)]
    #[serde(rename = "capacityProvider")]
    pub r#capacity_provider: Box<String>,
    /// The relative percentage of the total number of launched tasks that should use the specified capacity provider. The `weight` value is taken into consideration after the `base` count of tasks has been satisfied. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
