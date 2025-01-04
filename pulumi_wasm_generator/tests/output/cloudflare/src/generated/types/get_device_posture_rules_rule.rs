#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDevicePostureRulesRule {
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default)]
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<String>>,
    /// ID of the Device Posture Rule.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the device posture rule.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
