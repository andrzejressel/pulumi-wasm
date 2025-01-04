#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationMaximumCapacity {
    /// The maximum allowed CPU for an application.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<String>,
    /// The maximum allowed disk for an application.
    #[builder(into, default)]
    #[serde(rename = "disk")]
    pub r#disk: Box<Option<String>>,
    /// The maximum allowed resources for an application.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Box<String>,
}
