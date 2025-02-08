#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetLaunchTemplateInstanceMarketOptionSpotOption {
    #[builder(into)]
    #[serde(rename = "blockDurationMinutes")]
    pub r#block_duration_minutes: Box<i32>,
    #[builder(into)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: Box<String>,
    #[builder(into)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: Box<String>,
    #[builder(into)]
    #[serde(rename = "spotInstanceType")]
    pub r#spot_instance_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "validUntil")]
    pub r#valid_until: Box<String>,
}
