#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConnectorSubnet {
    /// Name of the resource.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Project in which the subnet exists. If not set, this project is assumed to be the project for which the connector create request was issued.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
}
