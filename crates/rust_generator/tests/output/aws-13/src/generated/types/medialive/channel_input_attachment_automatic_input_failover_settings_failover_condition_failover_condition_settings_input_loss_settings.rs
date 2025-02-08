#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings {
    /// The amount of time (in milliseconds) that no input is detected. After that time, an input failover will occur.
    #[builder(into, default)]
    #[serde(rename = "inputLossThresholdMsec")]
    pub r#input_loss_threshold_msec: Box<Option<i32>>,
}
