#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackupPlanBackupConfigEncryptionKey {
    /// Google Cloud KMS encryption key. Format: projects/*/locations/*/keyRings/*/cryptoKeys/*
    #[builder(into)]
    #[serde(rename = "gcpKmsEncryptionKey")]
    pub r#gcp_kms_encryption_key: Box<String>,
}
