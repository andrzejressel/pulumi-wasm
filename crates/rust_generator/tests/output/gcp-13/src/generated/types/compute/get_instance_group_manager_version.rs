#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceGroupManagerVersion {
    /// The full URL to an instance template from which all new instances of this version will be created.
    #[builder(into)]
    #[serde(rename = "instanceTemplate")]
    pub r#instance_template: Box<String>,
    /// The name of the instance group. Either `name` or `self_link` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The number of instances calculated as a fixed number or a percentage depending on the settings.
    #[builder(into)]
    #[serde(rename = "targetSizes")]
    pub r#target_sizes: Box<Vec<super::super::types::compute::GetInstanceGroupManagerVersionTargetSize>>,
}
