#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkspaceSettings {
    /// Project IDs for data projects hosted in a workspace.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "dataProjectIds")]
    pub r#data_project_ids: Box<Vec<String>>,
}
