#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSubscriptionAdvancedFilterIsNullOrUndefined {
    /// Specifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}