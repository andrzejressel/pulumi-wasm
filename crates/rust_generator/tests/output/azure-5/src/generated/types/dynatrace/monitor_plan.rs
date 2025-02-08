#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MonitorPlan {
    /// Different billing cycles. Possible values are `MONTHLY` and `WEEKLY`.
    #[builder(into, default)]
    #[serde(rename = "billingCycle")]
    pub r#billing_cycle: Box<Option<String>>,
    /// Date when plan was applied.
    #[builder(into, default)]
    #[serde(rename = "effectiveDate")]
    pub r#effective_date: Box<Option<String>>,
    /// Plan id as published by Dynatrace.
    #[builder(into)]
    #[serde(rename = "plan")]
    pub r#plan: Box<String>,
    /// Different usage type. Possible values are `PAYG` and `COMMITTED`.
    #[builder(into, default)]
    #[serde(rename = "usageType")]
    pub r#usage_type: Box<Option<String>>,
}
