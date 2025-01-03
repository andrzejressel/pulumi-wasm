#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceReadPoolConfig {
    /// Read capacity, i.e. number of nodes in a read pool instance.
    #[builder(into, default)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<Option<i32>>,
}
