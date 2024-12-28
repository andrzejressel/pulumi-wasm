#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomerSubscriptionDetails {
    /// Location placement Id of a subscription
    #[builder(into, default)]
    #[serde(rename = "locationPlacementId")]
    pub r#location_placement_id: Box<Option<String>>,
    /// Quota ID of a subscription
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: Box<String>,
    /// List of registered feature flags for subscription
    #[builder(into, default)]
    #[serde(rename = "registeredFeatures")]
    pub r#registered_features: Box<Option<Vec<super::types::CustomerSubscriptionRegisteredFeatures>>>,
}
