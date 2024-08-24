#[derive(serde::Serialize)]
pub struct GetDevicePostureRulesRule {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<String>>,
    /// ID of the Device Posture Rule.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the device posture rule.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
