#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobDefinitionEksProperties {
    /// Properties for the Kubernetes pod resources of a job. See `pod_properties` below.
    #[builder(into)]
    #[serde(rename = "podProperties")]
    pub r#pod_properties: Box<super::super::types::batch::JobDefinitionEksPropertiesPodProperties>,
}
