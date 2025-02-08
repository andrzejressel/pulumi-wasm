#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AlertPrometheusRuleGroupRuleAction {
    /// Specifies the resource id of the monitor action group.
    #[builder(into)]
    #[serde(rename = "actionGroupId")]
    pub r#action_group_id: Box<String>,
    /// Specifies the properties of an action group object.
    /// 
    /// > **Note:** `action_properties` can only be configured for IcM Connector Action Groups for now. Other public features will be supported in the future.
    #[builder(into, default)]
    #[serde(rename = "actionProperties")]
    pub r#action_properties: Box<Option<std::collections::HashMap<String, String>>>,
}
