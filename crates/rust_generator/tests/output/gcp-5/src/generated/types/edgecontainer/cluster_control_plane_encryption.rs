#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterControlPlaneEncryption {
    /// The Cloud KMS CryptoKey e.g.
    /// projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
    /// to use for protecting control plane disks. If not specified, a
    /// Google-managed key will be used instead.
    #[builder(into, default)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<Option<String>>,
    /// (Output)
    /// The Cloud KMS CryptoKeyVersion currently in use for protecting control
    /// plane disks. Only applicable if kms_key is set.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyActiveVersion")]
    pub r#kms_key_active_version: Box<Option<String>>,
    /// (Output)
    /// Availability of the Cloud KMS CryptoKey. If not `KEY_AVAILABLE`, then
    /// nodes may go offline as they cannot access their local data. This can be
    /// caused by a lack of permissions to use the key, or if the key is disabled
    /// or deleted.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyState")]
    pub r#kms_key_state: Box<Option<String>>,
    /// (Output)
    /// Error status returned by Cloud KMS when using this key. This field may be
    /// populated only if `kms_key_state` is not `KMS_KEY_STATE_KEY_AVAILABLE`.
    /// If populated, this field contains the error status reported by Cloud KMS.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_kms_status"></a>The `kms_status` block contains:
    #[builder(into, default)]
    #[serde(rename = "kmsStatuses")]
    pub r#kms_statuses: Box<Option<Vec<super::super::types::edgecontainer::ClusterControlPlaneEncryptionKmsStatus>>>,
}
