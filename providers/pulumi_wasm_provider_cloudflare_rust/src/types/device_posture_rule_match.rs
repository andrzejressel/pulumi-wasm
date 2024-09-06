#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DevicePostureRuleMatch {
    /// The platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`.
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}
