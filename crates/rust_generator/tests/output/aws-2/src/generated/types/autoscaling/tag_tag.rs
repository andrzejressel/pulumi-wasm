#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TagTag {
    /// Tag name.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Whether to propagate the tags to instances launched by the ASG.
    #[builder(into)]
    #[serde(rename = "propagateAtLaunch")]
    pub r#propagate_at_launch: Box<bool>,
    /// Tag value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
