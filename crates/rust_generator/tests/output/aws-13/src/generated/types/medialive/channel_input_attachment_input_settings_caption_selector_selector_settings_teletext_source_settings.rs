#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings {
    /// Optionally defines a region where TTML style captions will be displayed. See Caption Rectangle for more details.
    #[builder(into, default)]
    #[serde(rename = "outputRectangle")]
    pub r#output_rectangle: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettingsOutputRectangle>>,
    /// Specifies the teletext page number within the data stream from which to extract captions. Range of 0x100 (256) to 0x8FF (2303). Unused for passthrough. Should be specified as a hexadecimal string with no “0x” prefix.
    #[builder(into, default)]
    #[serde(rename = "pageNumber")]
    pub r#page_number: Box<Option<String>>,
}
