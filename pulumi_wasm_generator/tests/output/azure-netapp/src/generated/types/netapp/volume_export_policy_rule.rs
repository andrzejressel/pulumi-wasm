#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeExportPolicyRule {
    /// A list of allowed clients IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Box<Vec<String>>,
    /// Is Kerberos 5 read-only access permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "kerberos5ReadOnlyEnabled")]
    pub r#kerberos_5_read_only_enabled: Box<Option<bool>>,
    /// Is Kerberos 5 read/write permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "kerberos5ReadWriteEnabled")]
    pub r#kerberos_5_read_write_enabled: Box<Option<bool>>,
    /// Is Kerberos 5i read-only permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "kerberos5iReadOnlyEnabled")]
    pub r#kerberos_5_i_read_only_enabled: Box<Option<bool>>,
    /// Is Kerberos 5i read/write permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "kerberos5iReadWriteEnabled")]
    pub r#kerberos_5_i_read_write_enabled: Box<Option<bool>>,
    /// Is Kerberos 5p read-only permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "kerberos5pReadOnlyEnabled")]
    pub r#kerberos_5_p_read_only_enabled: Box<Option<bool>>,
    /// Is Kerberos 5p read/write permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "kerberos5pReadWriteEnabled")]
    pub r#kerberos_5_p_read_write_enabled: Box<Option<bool>>,
    /// A list of allowed protocols. Valid values include `CIFS`, `NFSv3`, or `NFSv4.1`. Only one value is supported at this time. This replaces the previous arguments: `cifs_enabled`, `nfsv3_enabled` and `nfsv4_enabled`.
    #[builder(into, default)]
    #[serde(rename = "protocolsEnabled")]
    pub r#protocols_enabled: Box<Option<String>>,
    /// Is root access permitted to this volume?
    #[builder(into, default)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Box<Option<bool>>,
    /// The index number of the rule.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: Box<i32>,
    /// Is the file system on unix read only?
    #[builder(into, default)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Box<Option<bool>>,
    /// Is the file system on unix read and write?
    #[builder(into, default)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Box<Option<bool>>,
}
