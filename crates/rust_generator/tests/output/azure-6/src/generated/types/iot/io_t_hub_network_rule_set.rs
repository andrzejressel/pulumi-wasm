#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubNetworkRuleSet {
    /// Determines if Network Rule Set is also applied to the BuiltIn EventHub EndPoint of the IotHub. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "applyToBuiltinEventhubEndpoint")]
    pub r#apply_to_builtin_eventhub_endpoint: Box<Option<bool>>,
    /// Default Action for Network Rule Set. Possible values are `DefaultActionDeny`, `DefaultActionAllow`. Defaults to `DefaultActionDeny`.
    #[builder(into, default)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<Option<String>>,
    /// One or more `ip_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<super::super::types::iot::IoTHubNetworkRuleSetIpRule>>>,
}
