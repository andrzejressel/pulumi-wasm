#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReservedCacheNodeRecurringCharge {
    #[builder(into)]
    #[serde(rename = "recurringChargeAmount")]
    pub r#recurring_charge_amount: Box<f64>,
    #[builder(into)]
    #[serde(rename = "recurringChargeFrequency")]
    pub r#recurring_charge_frequency: Box<String>,
}
