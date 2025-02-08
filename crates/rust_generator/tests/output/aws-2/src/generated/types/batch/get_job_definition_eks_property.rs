#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionEksProperty {
    /// The properties for the Kubernetes pod resources of a job.
    #[builder(into)]
    #[serde(rename = "podProperties")]
    pub r#pod_properties: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodProperty>>,
}
