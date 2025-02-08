#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourceCollectionTag {
    /// An AWS tag key that is used to identify the AWS resources that DevOps Guru analyzes.
    #[builder(into)]
    #[serde(rename = "appBoundaryKey")]
    pub r#app_boundary_key: Box<String>,
    /// Array of tag values.
    #[builder(into)]
    #[serde(rename = "tagValues")]
    pub r#tag_values: Box<Vec<String>>,
}
