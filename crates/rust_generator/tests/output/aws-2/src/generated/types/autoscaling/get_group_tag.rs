#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGroupTag {
    /// Key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Whether the tag is propagated to Amazon EC2 instances launched via this ASG.
    #[builder(into)]
    #[serde(rename = "propagateAtLaunch")]
    pub r#propagate_at_launch: Box<bool>,
    /// Value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
