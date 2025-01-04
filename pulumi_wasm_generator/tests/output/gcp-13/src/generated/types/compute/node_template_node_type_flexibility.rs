#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodeTemplateNodeTypeFlexibility {
    /// Number of virtual CPUs to use.
    #[builder(into, default)]
    #[serde(rename = "cpus")]
    pub r#cpus: Box<Option<String>>,
    /// (Output)
    /// Use local SSD
    #[builder(into, default)]
    #[serde(rename = "localSsd")]
    pub r#local_ssd: Box<Option<String>>,
    /// Physical memory available to the node, defined in MB.
    #[builder(into, default)]
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<String>>,
}
