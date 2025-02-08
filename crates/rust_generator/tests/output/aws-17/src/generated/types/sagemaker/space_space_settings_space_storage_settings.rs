#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpaceSpaceSettingsSpaceStorageSettings {
    /// A collection of EBS storage settings for a space. See `ebs_storage_settings` Block below.
    #[builder(into)]
    #[serde(rename = "ebsStorageSettings")]
    pub r#ebs_storage_settings: Box<super::super::types::sagemaker::SpaceSpaceSettingsSpaceStorageSettingsEbsStorageSettings>,
}
