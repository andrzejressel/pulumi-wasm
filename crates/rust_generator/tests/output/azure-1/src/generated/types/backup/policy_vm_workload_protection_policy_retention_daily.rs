#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyVmWorkloadProtectionPolicyRetentionDaily {
    /// The number of daily backups to keep. Possible values are between `7` and `9999`.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
}
