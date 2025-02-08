#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseCmekConfig {
    /// (Output)
    /// Currently in-use KMS key versions (https://cloud.google.com/kms/docs/resource-hierarchy#key_versions).
    /// During key rotation (https://cloud.google.com/kms/docs/key-rotation), there can be
    /// multiple in-use key versions.
    /// The expected format is
    /// `projects/{project_id}/locations/{kms_location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{key_version}`.
    #[builder(into, default)]
    #[serde(rename = "activeKeyVersions")]
    pub r#active_key_versions: Box<Option<Vec<String>>>,
    /// The resource ID of a Cloud KMS key. If set, the database created will
    /// be a Customer-managed Encryption Key (CMEK) database encrypted with
    /// this key. This feature is allowlist only in initial launch.
    /// Only keys in the same location as this database are allowed to be used
    /// for encryption. For Firestore's nam5 multi-region, this corresponds to Cloud KMS
    /// multi-region us. For Firestore's eur3 multi-region, this corresponds to
    /// Cloud KMS multi-region europe. See https://cloud.google.com/kms/docs/locations.
    /// This value should be the KMS key resource ID in the format of
    /// `projects/{project_id}/locations/{kms_location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
    /// How to retrive this resource ID is listed at
    /// https://cloud.google.com/kms/docs/getting-resource-ids#getting_the_id_for_a_key_and_version.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<String>,
}
