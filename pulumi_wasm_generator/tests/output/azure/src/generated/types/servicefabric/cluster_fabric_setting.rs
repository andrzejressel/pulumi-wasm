#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterFabricSetting {
    /// The name of the Fabric Setting, such as `Security` or `Federation`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A map containing settings for the specified Fabric Setting.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
}