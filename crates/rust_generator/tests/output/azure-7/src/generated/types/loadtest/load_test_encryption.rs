#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LoadTestEncryption {
    /// An `identity` block as defined below. Changing this forces a new Load Test to be created.
    #[builder(into)]
    #[serde(rename = "identity")]
    pub r#identity: Box<super::super::types::loadtest::LoadTestEncryptionIdentity>,
    /// The URI specifying the Key vault and key to be used to encrypt data in this resource. The URI should include the key version. Changing this forces a new Load Test to be created.
    #[builder(into)]
    #[serde(rename = "keyUrl")]
    pub r#key_url: Box<String>,
}
