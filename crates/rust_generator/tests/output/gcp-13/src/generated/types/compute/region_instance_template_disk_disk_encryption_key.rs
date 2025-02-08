#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionInstanceTemplateDiskDiskEncryptionKey {
    /// The self link of the encryption key that is stored in Google Cloud KMS
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<String>,
}
