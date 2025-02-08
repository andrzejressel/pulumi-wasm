#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourcePolicyInstanceSchedulePolicyVmStartSchedule {
    /// Specifies the frequency for the operation, using the unix-cron format.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<String>,
}
