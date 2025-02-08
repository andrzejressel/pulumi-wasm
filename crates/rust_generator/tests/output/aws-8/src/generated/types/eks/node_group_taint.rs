#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodeGroupTaint {
    /// The effect of the taint. Valid values: `NO_SCHEDULE`, `NO_EXECUTE`, `PREFER_NO_SCHEDULE`.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Box<String>,
    /// The key of the taint. Maximum length of 63.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value of the taint. Maximum length of 63.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
