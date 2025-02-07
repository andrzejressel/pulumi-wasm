#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVolumeGroupSapHanaVolumeExportPolicyRule {
    /// A list of allowed clients IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Box<String>,
    /// Is the NFSv3 protocol enabled?
    #[builder(into)]
    #[serde(rename = "nfsv3Enabled")]
    pub r#nfsv_3_enabled: Box<bool>,
    /// Is the NFSv4.1 enabled?
    #[builder(into)]
    #[serde(rename = "nfsv41Enabled")]
    pub r#nfsv_41_enabled: Box<bool>,
    /// Is root access permitted to this volume?
    #[builder(into)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Box<bool>,
    /// The index number of the rule.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: Box<i32>,
    /// Is the file system on unix read only?.
    #[builder(into)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Box<bool>,
    /// Is the file system on unix read and write?.
    #[builder(into)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Box<bool>,
}
