#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceDesiredPscAutoConnection {
    /// (Output)
    /// Output only. The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// (Output)
    /// Output only. The consumer project_id where the forwarding rule is created from.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
}
