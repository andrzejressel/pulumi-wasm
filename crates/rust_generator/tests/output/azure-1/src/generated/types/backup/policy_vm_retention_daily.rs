#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyVmRetentionDaily {
    /// The number of daily backups to keep. Must be between `7` and `9999`.
    /// 
    /// > **Note:** Azure previously allows this field to be set to a minimum of 1 (day) - but for new resources/to update this value on existing Backup Policies - this value must now be at least 7 (days).
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
}
