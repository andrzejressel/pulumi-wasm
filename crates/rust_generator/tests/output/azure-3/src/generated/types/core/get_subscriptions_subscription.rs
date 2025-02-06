#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSubscriptionsSubscription {
    /// The subscription display name.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// The ID of this subscription.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The subscription location placement ID.
    #[builder(into)]
    #[serde(rename = "locationPlacementId")]
    pub r#location_placement_id: Box<String>,
    /// The subscription quota ID.
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: Box<String>,
    /// The subscription spending limit.
    #[builder(into)]
    #[serde(rename = "spendingLimit")]
    pub r#spending_limit: Box<String>,
    /// The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// The subscription GUID.
    #[builder(into)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: Box<String>,
    /// A mapping of tags assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
    /// The subscription tenant ID.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
