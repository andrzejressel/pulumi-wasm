#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation {
    /// The ID of the file system, assigned by Amazon EFS.
    #[builder(into, default)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<Option<String>>,
    /// The pathname for the folder being used by a workflow.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}