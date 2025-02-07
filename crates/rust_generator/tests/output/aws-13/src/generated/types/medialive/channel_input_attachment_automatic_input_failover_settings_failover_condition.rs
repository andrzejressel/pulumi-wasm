#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition {
    /// Failover condition type-specific settings. See Failover Condition Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "failoverConditionSettings")]
    pub r#failover_condition_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings>>,
}
