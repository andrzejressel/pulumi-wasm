#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCollectionRuleDestinationLogAnalytic {
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of a Log Analytic Workspace resource.
    #[builder(into)]
    #[serde(rename = "workspaceResourceId")]
    pub r#workspace_resource_id: Box<String>,
}