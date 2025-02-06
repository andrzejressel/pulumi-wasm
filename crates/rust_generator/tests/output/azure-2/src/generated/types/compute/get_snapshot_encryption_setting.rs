#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSnapshotEncryptionSetting {
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeys")]
    pub r#disk_encryption_keys: Box<Vec<super::super::types::compute::GetSnapshotEncryptionSettingDiskEncryptionKey>>,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "keyEncryptionKeys")]
    pub r#key_encryption_keys: Box<Vec<super::super::types::compute::GetSnapshotEncryptionSettingKeyEncryptionKey>>,
}
