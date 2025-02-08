#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertProcessingRuleActionGroupCondition {
    /// A `alert_context` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "alertContext")]
    pub r#alert_context: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionAlertContext>>,
    /// A `alert_rule_id` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "alertRuleId")]
    pub r#alert_rule_id: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionAlertRuleId>>,
    /// A `alert_rule_name` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "alertRuleName")]
    pub r#alert_rule_name: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionAlertRuleName>>,
    /// A `description` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionDescription>>,
    /// A `monitor_condition` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "monitorCondition")]
    pub r#monitor_condition: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionMonitorCondition>>,
    /// A `monitor_service` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "monitorService")]
    pub r#monitor_service: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionMonitorService>>,
    /// A `severity` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "severity")]
    pub r#severity: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionSeverity>>,
    /// A `signal_type` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "signalType")]
    pub r#signal_type: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionSignalType>>,
    /// A `target_resource` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "targetResource")]
    pub r#target_resource: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionTargetResource>>,
    /// A `target_resource_group` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "targetResourceGroup")]
    pub r#target_resource_group: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionTargetResourceGroup>>,
    /// A `target_resource_type` block as defined below.
    /// 
    /// > **Note:** At least one of the `alert_context`, `alert_rule_id`, `alert_rule_name`, `description`, `monitor_condition`, `monitor_service`, `severity`, `signal_type`, `target_resource`, `target_resource_group`, `target_resource_type` must be specified.
    #[builder(into, default)]
    #[serde(rename = "targetResourceType")]
    pub r#target_resource_type: Box<Option<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionTargetResourceType>>,
}
