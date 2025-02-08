#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpaceSpaceSettingsSpaceStorageSettingsEbsStorageSettings {
    /// The size of an EBS storage volume for a space.
    #[builder(into)]
    #[serde(rename = "ebsVolumeSizeInGb")]
    pub r#ebs_volume_size_in_gb: Box<i32>,
}
