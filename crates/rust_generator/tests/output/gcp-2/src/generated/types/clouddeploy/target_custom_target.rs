#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TargetCustomTarget {
    /// Required. The name of the CustomTargetType. Format must be `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`.
    #[builder(into)]
    #[serde(rename = "customTargetType")]
    pub r#custom_target_type: Box<String>,
}
