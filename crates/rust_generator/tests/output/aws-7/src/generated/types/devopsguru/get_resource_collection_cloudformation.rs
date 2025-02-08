#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetResourceCollectionCloudformation {
    /// Array of the names of the AWS CloudFormation stacks.
    #[builder(into)]
    #[serde(rename = "stackNames")]
    pub r#stack_names: Box<Vec<String>>,
}
