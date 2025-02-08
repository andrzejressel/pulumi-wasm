#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings {
    /// Specify the canned ACL to apply to each S3 request.
    #[builder(into, default)]
    #[serde(rename = "cannedAcl")]
    pub r#canned_acl: Box<Option<String>>,
}
