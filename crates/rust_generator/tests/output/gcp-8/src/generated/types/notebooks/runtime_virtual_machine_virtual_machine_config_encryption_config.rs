#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuntimeVirtualMachineVirtualMachineConfigEncryptionConfig {
    /// The Cloud KMS resource identifier of the customer-managed
    /// encryption key used to protect a resource, such as a disks.
    /// It has the following format:
    /// `projects/{PROJECT_ID}/locations/{REGION}/keyRings/
    /// {KEY_RING_NAME}/cryptoKeys/{KEY_NAME}`
    #[builder(into, default)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<Option<String>>,
}
