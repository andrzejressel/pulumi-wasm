#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNodeGroupScalingConfig {
    /// Desired number of worker nodes.
    #[builder(into)]
    #[serde(rename = "desiredSize")]
    pub r#desired_size: Box<i32>,
    /// Maximum number of worker nodes.
    #[builder(into)]
    #[serde(rename = "maxSize")]
    pub r#max_size: Box<i32>,
    /// Minimum number of worker nodes.
    #[builder(into)]
    #[serde(rename = "minSize")]
    pub r#min_size: Box<i32>,
}
