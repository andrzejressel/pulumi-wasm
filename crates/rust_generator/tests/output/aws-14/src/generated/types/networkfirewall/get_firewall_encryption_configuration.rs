#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFirewallEncryptionConfiguration {
    /// The ID of the AWS Key Management Service (AWS KMS) customer managed key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<String>,
    /// The type of the AWS Key Management Service (AWS KMS) key use by the firewall.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
