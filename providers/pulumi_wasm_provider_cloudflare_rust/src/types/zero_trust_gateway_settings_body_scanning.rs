#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsBodyScanning {
    /// Body scanning inspection mode. Available values: `deep`, `shallow`.
    #[builder(into)]
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}
