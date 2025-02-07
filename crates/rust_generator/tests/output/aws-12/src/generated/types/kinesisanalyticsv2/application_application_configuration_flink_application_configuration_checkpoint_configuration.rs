#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfigurationCheckpointConfiguration {
    /// Describes the interval in milliseconds between checkpoint operations.
    #[builder(into, default)]
    #[serde(rename = "checkpointInterval")]
    pub r#checkpoint_interval: Box<Option<i32>>,
    /// Describes whether checkpointing is enabled for a Flink-based Kinesis Data Analytics application.
    #[builder(into, default)]
    #[serde(rename = "checkpointingEnabled")]
    pub r#checkpointing_enabled: Box<Option<bool>>,
    /// Describes whether the application uses Kinesis Data Analytics' default checkpointing behavior. Valid values: `CUSTOM`, `DEFAULT`. Set this attribute to `CUSTOM` in order for any specified `checkpointing_enabled`, `checkpoint_interval`, or `min_pause_between_checkpoints` attribute values to be effective. If this attribute is set to `DEFAULT`, the application will always use the following values:
    /// * `checkpointing_enabled = true`
    /// * `checkpoint_interval = 60000`
    /// * `min_pause_between_checkpoints = 5000`
    #[builder(into)]
    #[serde(rename = "configurationType")]
    pub r#configuration_type: Box<String>,
    /// Describes the minimum time in milliseconds after a checkpoint operation completes that a new checkpoint operation can start.
    #[builder(into, default)]
    #[serde(rename = "minPauseBetweenCheckpoints")]
    pub r#min_pause_between_checkpoints: Box<Option<i32>>,
}
