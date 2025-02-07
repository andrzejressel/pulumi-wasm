#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyFileShareRetentionDaily {
    /// The number of daily backups to keep. Must be between `1` and `200` (inclusive)
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
}
