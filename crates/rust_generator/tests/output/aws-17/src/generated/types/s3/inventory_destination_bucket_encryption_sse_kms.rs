#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InventoryDestinationBucketEncryptionSseKms {
    /// ARN of the KMS customer master key (CMK) used to encrypt the inventory file.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<String>,
}
