#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GrafanaAzureMonitorWorkspaceIntegration {
    /// Specifies the resource ID of the connected Azure Monitor Workspace.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
}