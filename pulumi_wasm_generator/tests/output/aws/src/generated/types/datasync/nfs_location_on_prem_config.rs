#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NfsLocationOnPremConfig {
    /// List of Amazon Resource Names (ARNs) of the DataSync Agents used to connect to the NFS server.
    #[builder(into)]
    #[serde(rename = "agentArns")]
    pub r#agent_arns: Box<Vec<String>>,
}
