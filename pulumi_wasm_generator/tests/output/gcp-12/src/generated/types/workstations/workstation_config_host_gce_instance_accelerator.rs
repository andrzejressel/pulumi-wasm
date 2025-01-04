#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigHostGceInstanceAccelerator {
    /// Number of accelerator cards exposed to the instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// Type of accelerator resource to attach to the instance, for example, "nvidia-tesla-p100".
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
