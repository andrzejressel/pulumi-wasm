#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecuritySolutionAdditionalWorkspace {
    /// A list of data types which sent to workspace. Possible values are `Alerts` and `RawEvents`.
    #[builder(into)]
    #[serde(rename = "dataTypes")]
    pub r#data_types: Box<Vec<String>>,
    /// The resource ID of the Log Analytics Workspace.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: Box<String>,
}