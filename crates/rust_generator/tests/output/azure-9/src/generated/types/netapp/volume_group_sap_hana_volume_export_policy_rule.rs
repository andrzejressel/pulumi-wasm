#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeGroupSapHanaVolumeExportPolicyRule {
    /// A comma-sperated list of allowed client IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Box<String>,
    /// Enables NFSv3. Please note that this cannot be enabled if volume has NFSv4.1 as its protocol.
    #[builder(into)]
    #[serde(rename = "nfsv3Enabled")]
    pub r#nfsv_3_enabled: Box<bool>,
    /// Enables NFSv4.1. Please note that this cannot be enabled if volume has NFSv3 as its protocol.
    #[builder(into)]
    #[serde(rename = "nfsv41Enabled")]
    pub r#nfsv_41_enabled: Box<bool>,
    /// Is root access permitted to this volume? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Box<Option<bool>>,
    /// The index number of the rule, must start at 1 and maximum 5.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: Box<i32>,
    /// Is the file system on unix read only? Defaults to `false.
    #[builder(into, default)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Box<Option<bool>>,
    /// Is the file system on unix read and write? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Box<Option<bool>>,
}
