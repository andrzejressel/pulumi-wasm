#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitorPlan {
    /// Specifies the billing cycles. Possible values are `MONTHLY`, `WEEKLY` and `YEARLY`. Defaults to `MONTHLY`. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "billingCycle")]
    pub r#billing_cycle: Box<Option<String>>,
    /// Specifies the date when plan was applied. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into)]
    #[serde(rename = "effectiveDate")]
    pub r#effective_date: Box<String>,
    /// Specifies the plan id published by NewRelic. The only possible value is `newrelic-pay-as-you-go-free-live`. Defaults to `newrelic-pay-as-you-go-free-live`. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "planId")]
    pub r#plan_id: Box<Option<String>>,
    /// Specifies the usage type. Possible values are `COMMITTED` and `PAYG`. Defaults to `PAYG`. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "usageType")]
    pub r#usage_type: Box<Option<String>>,
}
