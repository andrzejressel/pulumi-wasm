#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioTrackSelectionDolbyEDecode {
    /// Applies only to Dolby E. Enter the program ID (according to the metadata in the audio) of the Dolby E program to extract from the specified track. One program extracted per audio selector. To select multiple programs, create multiple selectors with the same Track and different Program numbers. “All channels” means to ignore the program IDs and include all the channels in this selector; useful if metadata is known to be incorrect.
    #[builder(into)]
    #[serde(rename = "programSelection")]
    pub r#program_selection: Box<String>,
}
