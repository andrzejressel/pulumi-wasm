#[derive(serde::Serialize)]
pub struct TunnelConfigConfigWarpRouting {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
