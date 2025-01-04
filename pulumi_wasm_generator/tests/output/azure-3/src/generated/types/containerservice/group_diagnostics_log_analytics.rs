#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupDiagnosticsLogAnalytics {
    /// The log type which should be used. Possible values are `ContainerInsights` and `ContainerInstanceLogs`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<Option<String>>,
    /// Any metadata required for Log Analytics. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Workspace ID of the Log Analytics Workspace. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: Box<String>,
    /// The Workspace Key of the Log Analytics Workspace. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "workspaceKey")]
    pub r#workspace_key: Box<String>,
}
