#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkteamWorkerAccessConfiguration {
    /// Defines any Amazon S3 resource constraints. see S3 Presign details below.
    #[builder(into, default)]
    #[serde(rename = "s3Presign")]
    pub r#s_3_presign: Box<Option<super::super::types::sagemaker::WorkteamWorkerAccessConfigurationS3Presign>>,
}
