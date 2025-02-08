#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerUlimit {
    /// The hard limit for the ulimit type.
    #[builder(into)]
    #[serde(rename = "hardLimit")]
    pub r#hard_limit: Box<i32>,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The soft limit for the ulimit type.
    #[builder(into)]
    #[serde(rename = "softLimit")]
    pub r#soft_limit: Box<i32>,
}
