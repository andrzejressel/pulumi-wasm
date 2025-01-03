#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuntimeMetric {
    /// (Output)
    /// Contains runtime daemon metrics, such as OS and kernels and
    /// sessions stats.
    #[builder(into, default)]
    #[serde(rename = "systemMetrics")]
    pub r#system_metrics: Box<Option<std::collections::HashMap<String, String>>>,
}
