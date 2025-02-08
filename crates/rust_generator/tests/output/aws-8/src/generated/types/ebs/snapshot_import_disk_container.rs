#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SnapshotImportDiskContainer {
    /// The description of the disk image being imported.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The format of the disk image being imported. One of `VHD` or `VMDK`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// The URL to the Amazon S3-based disk image being imported. It can either be a https URL (https://..) or an Amazon S3 URL (s3://..). One of `url` or `user_bucket` must be set.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
    /// The Amazon S3 bucket for the disk image. One of `url` or `user_bucket` must be set. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "userBucket")]
    pub r#user_bucket: Box<Option<super::super::types::ebs::SnapshotImportDiskContainerUserBucket>>,
}
