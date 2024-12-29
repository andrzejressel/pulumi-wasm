#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventDestinationCloudwatchDestination {
    /// The default value for the event
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<String>,
    /// The name for the dimension
    #[builder(into)]
    #[serde(rename = "dimensionName")]
    pub r#dimension_name: Box<String>,
    /// The source for the value. May be any of `"messageTag"`, `"emailHeader"` or `"linkTag"`.
    #[builder(into)]
    #[serde(rename = "valueSource")]
    pub r#value_source: Box<String>,
}
