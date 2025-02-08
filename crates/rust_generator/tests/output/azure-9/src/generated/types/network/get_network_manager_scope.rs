#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkManagerScope {
    /// A list of management group IDs used a scope for the Network Manager.
    #[builder(into)]
    #[serde(rename = "managementGroupIds")]
    pub r#management_group_ids: Box<Vec<String>>,
    /// A list of subscription IDs used as the scope for the Network Manager.
    #[builder(into)]
    #[serde(rename = "subscriptionIds")]
    pub r#subscription_ids: Box<Vec<String>>,
}
