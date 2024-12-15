//! Represents subscription registered features

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomerSubscriptionRegisteredFeatures {
    /// Name of subscription registered feature
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// State of subscription registered feature
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
