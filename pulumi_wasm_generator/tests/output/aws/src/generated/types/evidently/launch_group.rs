#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchGroup {
    /// Specifies the description of the launch group.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Specifies the name of the feature that the launch is using.
    #[builder(into)]
    #[serde(rename = "feature")]
    pub r#feature: Box<String>,
    /// Specifies the name of the lahnch group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the feature variation to use for this launch group.
    #[builder(into)]
    #[serde(rename = "variation")]
    pub r#variation: Box<String>,
}