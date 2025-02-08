#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacementManagedClusterConfigInitializationAction {
    /// Required. Cloud Storage URI of executable file.
    #[builder(into, default)]
    #[serde(rename = "executableFile")]
    pub r#executable_file: Box<Option<String>>,
    /// Amount of time executable has to complete. Default is 10 minutes (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)). Cluster creation fails with an explanatory error message (the name of the executable that caused the error and the exceeded timeout period) if the executable is not completed at end of the timeout period.
    #[builder(into, default)]
    #[serde(rename = "executionTimeout")]
    pub r#execution_timeout: Box<Option<String>>,
}
