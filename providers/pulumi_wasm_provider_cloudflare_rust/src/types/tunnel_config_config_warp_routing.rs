#[derive(serde::Serialize)]
pub struct TunnelConfigConfigWarpRouting {
    /// Whether WARP routing is enabled.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
