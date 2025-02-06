#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodeConfigSandboxConfig {
    /// Type of the sandbox to use for the node (e.g. 'gvisor')
    #[builder(into)]
    #[serde(rename = "sandboxType")]
    pub r#sandbox_type: Box<String>,
}
