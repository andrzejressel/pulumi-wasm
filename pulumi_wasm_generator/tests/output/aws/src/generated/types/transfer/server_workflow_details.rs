#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServerWorkflowDetails {
    /// A trigger that starts a workflow if a file is only partially uploaded. See Workflow Detail below. See `on_partial_upload` Block below for details.
    #[builder(into, default)]
    #[serde(rename = "onPartialUpload")]
    pub r#on_partial_upload: Box<Option<super::super::types::transfer::ServerWorkflowDetailsOnPartialUpload>>,
    /// A trigger that starts a workflow: the workflow begins to execute after a file is uploaded. See `on_upload` Block below for details.
    #[builder(into, default)]
    #[serde(rename = "onUpload")]
    pub r#on_upload: Box<Option<super::super::types::transfer::ServerWorkflowDetailsOnUpload>>,
}