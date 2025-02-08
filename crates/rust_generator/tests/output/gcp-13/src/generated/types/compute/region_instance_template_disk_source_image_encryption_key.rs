#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionInstanceTemplateDiskSourceImageEncryptionKey {
    /// The self link of the encryption key that is
    /// stored in Google Cloud KMS.
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<String>,
    /// The service account being used for the
    /// encryption request for the given KMS key. If absent, the Compute Engine
    /// default service account is used.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyServiceAccount")]
    pub r#kms_key_service_account: Box<Option<String>>,
}
