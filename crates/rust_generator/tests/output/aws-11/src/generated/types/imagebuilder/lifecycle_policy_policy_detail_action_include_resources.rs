#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailActionIncludeResources {
    /// Specifies whether the lifecycle action should apply to distributed AMIs.
    #[builder(into, default)]
    #[serde(rename = "amis")]
    pub r#amis: Box<Option<bool>>,
    /// Specifies whether the lifecycle action should apply to distributed containers.
    #[builder(into, default)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Option<bool>>,
    /// Specifies whether the lifecycle action should apply to snapshots associated with distributed AMIs.
    #[builder(into, default)]
    #[serde(rename = "snapshots")]
    pub r#snapshots: Box<Option<bool>>,
}
