#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReceivedLicenseConsumptionConfigurationProvisionalConfiguration {
    /// Maximum time for the provisional configuration, in minutes.
    #[builder(into)]
    #[serde(rename = "maxTimeToLiveInMinutes")]
    pub r#max_time_to_live_in_minutes: Box<i32>,
}
