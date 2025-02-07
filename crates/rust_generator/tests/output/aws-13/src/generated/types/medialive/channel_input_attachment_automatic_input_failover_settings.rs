#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettings {
    /// This clear time defines the requirement a recovered input must meet to be considered healthy. The input must have no failover conditions for this length of time. Enter a time in milliseconds. This value is particularly important if the input\_preference for the failover pair is set to PRIMARY\_INPUT\_PREFERRED, because after this time, MediaLive will switch back to the primary input.
    #[builder(into, default)]
    #[serde(rename = "errorClearTimeMsec")]
    pub r#error_clear_time_msec: Box<Option<i32>>,
    /// A list of failover conditions. If any of these conditions occur, MediaLive will perform a failover to the other input. See Failover Condition Block for more details.
    #[builder(into, default)]
    #[serde(rename = "failoverConditions")]
    pub r#failover_conditions: Box<Option<Vec<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition>>>,
    /// Input preference when deciding which input to make active when a previously failed input has recovered.
    #[builder(into, default)]
    #[serde(rename = "inputPreference")]
    pub r#input_preference: Box<Option<String>>,
    /// The input ID of the secondary input in the automatic input failover pair.
    #[builder(into)]
    #[serde(rename = "secondaryInputId")]
    pub r#secondary_input_id: Box<String>,
}
