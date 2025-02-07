#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkPolicyInternetAccess {
    /// True if the service is enabled; false otherwise.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// State of the service. New values may be added to this enum when appropriate.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
