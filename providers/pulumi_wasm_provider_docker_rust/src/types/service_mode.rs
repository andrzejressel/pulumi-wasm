#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceMode {
    /// When `true`, tasks will run on every worker node. Conflicts with `replicated`
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    /// The replicated service mode
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<crate::types::ServiceModeReplicated>>,
}
