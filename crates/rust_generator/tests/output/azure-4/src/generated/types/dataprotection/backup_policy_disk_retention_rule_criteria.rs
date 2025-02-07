#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPolicyDiskRetentionRuleCriteria {
    /// Possible values are `FirstOfDay` and `FirstOfWeek`. Changing this forces a new Backup Policy Disk to be created.
    #[builder(into, default)]
    #[serde(rename = "absoluteCriteria")]
    pub r#absolute_criteria: Box<Option<String>>,
}
