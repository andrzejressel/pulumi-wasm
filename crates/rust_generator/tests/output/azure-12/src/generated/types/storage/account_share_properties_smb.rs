#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountSharePropertiesSmb {
    /// A set of SMB authentication methods. Possible values are `NTLMv2`, and `Kerberos`.
    #[builder(into, default)]
    #[serde(rename = "authenticationTypes")]
    pub r#authentication_types: Box<Option<Vec<String>>>,
    /// A set of SMB channel encryption. Possible values are `AES-128-CCM`, `AES-128-GCM`, and `AES-256-GCM`.
    #[builder(into, default)]
    #[serde(rename = "channelEncryptionTypes")]
    pub r#channel_encryption_types: Box<Option<Vec<String>>>,
    /// A set of Kerberos ticket encryption. Possible values are `RC4-HMAC`, and `AES-256`.
    #[builder(into, default)]
    #[serde(rename = "kerberosTicketEncryptionTypes")]
    pub r#kerberos_ticket_encryption_types: Box<Option<Vec<String>>>,
    /// Indicates whether multichannel is enabled. Defaults to `false`. This is only supported on Premium storage accounts.
    #[builder(into, default)]
    #[serde(rename = "multichannelEnabled")]
    pub r#multichannel_enabled: Box<Option<bool>>,
    /// A set of SMB protocol versions. Possible values are `SMB2.1`, `SMB3.0`, and `SMB3.1.1`.
    #[builder(into, default)]
    #[serde(rename = "versions")]
    pub r#versions: Box<Option<Vec<String>>>,
}
