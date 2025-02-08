#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchTemplateBlockDeviceMappingEb {
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<String>,
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<String>,
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Box<i32>,
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<i32>,
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<i32>,
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<String>,
}
