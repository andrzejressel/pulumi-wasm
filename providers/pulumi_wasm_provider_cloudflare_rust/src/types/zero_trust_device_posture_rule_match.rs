#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDevicePostureRuleMatch {
    /// The platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}
