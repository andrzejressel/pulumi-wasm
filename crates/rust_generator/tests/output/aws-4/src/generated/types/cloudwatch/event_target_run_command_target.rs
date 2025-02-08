#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetRunCommandTarget {
    /// Can be either `tag:tag-key` or `InstanceIds`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// If Key is `tag:tag-key`, Values is a list of tag values. If Key is `InstanceIds`, Values is a list of Amazon EC2 instance IDs.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
