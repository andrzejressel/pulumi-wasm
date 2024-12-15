//! Holds Customer subscription details. Clients can display available products to unregistered customers by explicitly passing subscription details

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomerSubscriptionDetails {
    /// Location placement Id of a subscription
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "locationPlacementId")]
    pub r#location_placement_id: Box<Option<String>>,
    /// Quota ID of a subscription
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: Box<String>,
    /// List of registered feature flags for subscription
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "registeredFeatures")]
    pub r#registered_features: Box<Option<Vec<crate::types::CustomerSubscriptionRegisteredFeatures>>>,
}
