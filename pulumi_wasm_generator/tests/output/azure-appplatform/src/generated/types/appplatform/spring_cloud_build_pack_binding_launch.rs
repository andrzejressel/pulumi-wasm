#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudBuildPackBindingLaunch {
    /// Specifies a map of non-sensitive properties for launchProperties.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies a map of sensitive properties for launchProperties.
    #[builder(into, default)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<std::collections::HashMap<String, String>>>,
}