#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSecurity {
    /// Whether to enable Kerberos Armoring. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "kerberosArmoringEnabled")]
    pub r#kerberos_armoring_enabled: Box<Option<bool>>,
    /// Whether to enable Kerberos RC4 Encryption. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "kerberosRc4EncryptionEnabled")]
    pub r#kerberos_rc_4_encryption_enabled: Box<Option<bool>>,
    /// Whether to enable legacy NTLM v1 support. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ntlmV1Enabled")]
    pub r#ntlm_v_1_enabled: Box<Option<bool>>,
    /// Whether to synchronize Kerberos password hashes to the managed domain. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "syncKerberosPasswords")]
    pub r#sync_kerberos_passwords: Box<Option<bool>>,
    /// Whether to synchronize NTLM password hashes to the managed domain. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "syncNtlmPasswords")]
    pub r#sync_ntlm_passwords: Box<Option<bool>>,
    /// Whether to synchronize on-premises password hashes to the managed domain. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "syncOnPremPasswords")]
    pub r#sync_on_prem_passwords: Box<Option<bool>>,
    /// Whether to enable legacy TLS v1 support. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "tlsV1Enabled")]
    pub r#tls_v_1_enabled: Box<Option<bool>>,
}
