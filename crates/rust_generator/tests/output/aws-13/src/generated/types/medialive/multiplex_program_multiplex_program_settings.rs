#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MultiplexProgramMultiplexProgramSettings {
    /// Enum for preferred channel pipeline. Options are `CURRENTLY_ACTIVE`, `PIPELINE_0`, or `PIPELINE_1`.
    #[builder(into)]
    #[serde(rename = "preferredChannelPipeline")]
    pub r#preferred_channel_pipeline: Box<String>,
    /// Unique program number.
    #[builder(into)]
    #[serde(rename = "programNumber")]
    pub r#program_number: Box<i32>,
    /// Service Descriptor. See Service Descriptor for more details.
    #[builder(into, default)]
    #[serde(rename = "serviceDescriptor")]
    pub r#service_descriptor: Box<Option<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsServiceDescriptor>>,
    /// Video settings. See Video Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "videoSettings")]
    pub r#video_settings: Box<Option<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettings>>,
}
