#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReservedInstanceRecurringCharge {
    #[builder(into, default)]
    #[serde(rename = "recurringChargeAmount")]
    pub r#recurring_charge_amount: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "recurringChargeFrequency")]
    pub r#recurring_charge_frequency: Box<Option<String>>,
}
