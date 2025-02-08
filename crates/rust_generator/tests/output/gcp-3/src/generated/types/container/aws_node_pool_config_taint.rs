#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AwsNodePoolConfigTaint {
    /// The taint effect. Possible values: EFFECT_UNSPECIFIED, NO_SCHEDULE, PREFER_NO_SCHEDULE, NO_EXECUTE
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Box<String>,
    /// Key for the taint.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Value for the taint.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
