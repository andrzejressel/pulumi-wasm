#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClientCloudKmsConfig {
    /// A Cloud KMS key is a named object containing one or more key versions, along
    /// with metadata for the key. A key exists on exactly one key ring tied to a
    /// specific location.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Each version of a key contains key material used for encryption or signing.
    /// A key's version is represented by an integer, starting at 1. To decrypt data
    /// or verify a signature, you must use the same key version that was used to
    /// encrypt or sign the data.
    #[builder(into, default)]
    #[serde(rename = "keyVersion")]
    pub r#key_version: Box<Option<String>>,
    /// Location name of the key ring, e.g. "us-west1".
    #[builder(into)]
    #[serde(rename = "kmsLocation")]
    pub r#kms_location: Box<String>,
    /// The Google Cloud project id of the project where the kms key stored. If empty,
    /// the kms key is stored at the same project as customer's project and ecrypted
    /// with CMEK, otherwise, the kms key is stored in the tenant project and
    /// encrypted with GMEK.
    #[builder(into, default)]
    #[serde(rename = "kmsProjectId")]
    pub r#kms_project_id: Box<Option<String>>,
    /// A key ring organizes keys in a specific Google Cloud location and allows you to
    /// manage access control on groups of keys. A key ring's name does not need to be
    /// unique across a Google Cloud project, but must be unique within a given location.
    #[builder(into)]
    #[serde(rename = "kmsRing")]
    pub r#kms_ring: Box<String>,
}
