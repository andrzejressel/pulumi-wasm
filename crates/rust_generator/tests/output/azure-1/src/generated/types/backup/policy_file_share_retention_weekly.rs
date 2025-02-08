#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyFileShareRetentionWeekly {
    /// The number of daily backups to keep. Must be between `1` and `200` (inclusive)
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The weekday backups to retain. Must be one of `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Box<Vec<String>>,
}
