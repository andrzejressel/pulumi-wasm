#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatasetEncryptionSpec {
    /// KMS encryption key that is used to secure this dataset and its sub-resources. The key used for
    /// encryption and the dataset must be in the same location. If empty, the default Google encryption
    /// key will be used to secure this dataset. The format is
    /// projects/{projectId}/locations/{locationId}/keyRings/{keyRingId}/cryptoKeys/{keyId}.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
}
