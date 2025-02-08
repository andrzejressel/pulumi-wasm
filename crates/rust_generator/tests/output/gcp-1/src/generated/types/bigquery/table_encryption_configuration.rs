#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableEncryptionConfiguration {
    /// The self link or full name of a key which should be used to
    /// encrypt this table.  Note that the default bigquery service account will need to have
    /// encrypt/decrypt permissions on this key - you may want to see the
    /// `gcp.bigquery.getDefaultServiceAccount` datasource and the
    /// `gcp.kms.CryptoKeyIAMBinding` resource.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<String>,
    /// The self link or full name of the kms key version used to encrypt this table.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyVersion")]
    pub r#kms_key_version: Box<Option<String>>,
}
