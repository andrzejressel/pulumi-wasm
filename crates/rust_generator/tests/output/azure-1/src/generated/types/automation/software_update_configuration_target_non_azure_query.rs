#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SoftwareUpdateConfigurationTargetNonAzureQuery {
    /// Specifies the Log Analytics save search name.
    #[builder(into, default)]
    #[serde(rename = "functionAlias")]
    pub r#function_alias: Box<Option<String>>,
    /// The workspace id for Log Analytics in which the saved search in.
    #[builder(into, default)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: Box<Option<String>>,
}
