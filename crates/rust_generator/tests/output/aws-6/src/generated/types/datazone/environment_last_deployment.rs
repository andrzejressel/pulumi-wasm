#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentLastDeployment {
    #[builder(into)]
    #[serde(rename = "deploymentId")]
    pub r#deployment_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "deploymentStatus")]
    pub r#deployment_status: Box<String>,
    #[builder(into)]
    #[serde(rename = "deploymentType")]
    pub r#deployment_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "failureReasons")]
    pub r#failure_reasons: Box<Vec<super::super::types::datazone::EnvironmentLastDeploymentFailureReason>>,
    #[builder(into)]
    #[serde(rename = "isDeploymentComplete")]
    pub r#is_deployment_complete: Box<bool>,
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Box<Vec<String>>,
}
