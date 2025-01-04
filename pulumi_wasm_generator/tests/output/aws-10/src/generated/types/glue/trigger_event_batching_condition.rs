#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerEventBatchingCondition {
    /// Number of events that must be received from Amazon EventBridge before EventBridge  event trigger fires.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Box<i32>,
    /// Window of time in seconds after which EventBridge event trigger fires. Window starts when first event is received. Default value is `900`.
    #[builder(into, default)]
    #[serde(rename = "batchWindow")]
    pub r#batch_window: Box<Option<i32>>,
}
