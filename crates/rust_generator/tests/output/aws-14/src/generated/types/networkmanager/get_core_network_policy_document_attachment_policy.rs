#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCoreNetworkPolicyDocumentAttachmentPolicy {
    /// Action to take when a condition is true. Detailed Below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentAttachmentPolicyAction>,
    /// Valid values include `and` or `or`. This is a mandatory parameter only if you have more than one condition. The `condition_logic` apply to all of the conditions for a rule, which also means nested conditions of `and` or `or` are not supported. Use `or` if you want to associate the attachment with the segment by either the segment name or attachment tag value, or by the chosen conditions. Use `and` if you want to associate the attachment with the segment by either the segment name or attachment tag value and by the chosen conditions. Detailed Below.
    #[builder(into, default)]
    #[serde(rename = "conditionLogic")]
    pub r#condition_logic: Box<Option<String>>,
    /// A block argument. Detailed Below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentAttachmentPolicyCondition>>,
    /// A user-defined description that further helps identify the rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// An integer from `1` to `65535` indicating the rule's order number. Rules are processed in order from the lowest numbered rule to the highest. Rules stop processing when a rule is matched. It's important to make sure that you number your rules in the exact order that you want them processed.
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: Box<i32>,
}
