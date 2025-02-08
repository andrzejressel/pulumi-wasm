#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceCollectionCloudformation {
    /// Array of the names of the AWS CloudFormation stacks. If `type` is `AWS_SERVICE` (all acccount resources) this array should be a single item containing a wildcard (`"*"`).
    #[builder(into)]
    #[serde(rename = "stackNames")]
    pub r#stack_names: Box<Vec<String>>,
}
