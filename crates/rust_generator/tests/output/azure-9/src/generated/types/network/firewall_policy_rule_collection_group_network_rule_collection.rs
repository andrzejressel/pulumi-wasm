#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupNetworkRuleCollection {
    /// The action to take for the network rules in this collection. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The name which should be used for this network rule collection.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The priority of the network rule collection. The range is `100` - `65000`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// One or more `network_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::network::FirewallPolicyRuleCollectionGroupNetworkRuleCollectionRule>>,
}
