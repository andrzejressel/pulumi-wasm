#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceSecurity {
    /// (Optional) Whether the Kerberos Armoring is enabled.
    #[builder(into)]
    #[serde(rename = "kerberosArmoringEnabled")]
    pub r#kerberos_armoring_enabled: Box<bool>,
    /// (Optional) Whether the Kerberos RC4 Encryption is enabled.
    #[builder(into)]
    #[serde(rename = "kerberosRc4EncryptionEnabled")]
    pub r#kerberos_rc_4_encryption_enabled: Box<bool>,
    /// Whether legacy NTLM v1 support is enabled.
    #[builder(into)]
    #[serde(rename = "ntlmV1Enabled")]
    pub r#ntlm_v_1_enabled: Box<bool>,
    /// Whether Kerberos password hashes are synchronized to the managed domain.
    #[builder(into)]
    #[serde(rename = "syncKerberosPasswords")]
    pub r#sync_kerberos_passwords: Box<bool>,
    /// Whether NTLM password hashes are synchronized to the managed domain.
    #[builder(into)]
    #[serde(rename = "syncNtlmPasswords")]
    pub r#sync_ntlm_passwords: Box<bool>,
    /// Whether on-premises password hashes are synchronized to the managed domain.
    #[builder(into)]
    #[serde(rename = "syncOnPremPasswords")]
    pub r#sync_on_prem_passwords: Box<bool>,
    /// Whether legacy TLS v1 support is enabled.
    #[builder(into)]
    #[serde(rename = "tlsV1Enabled")]
    pub r#tls_v_1_enabled: Box<bool>,
}
