#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceMode {
    /// When `true`, tasks will run on every worker node. Conflicts with `replicated`
    #[builder(into, default)]
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    /// The replicated service mode
    #[builder(into, default)]
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<super::types::ServiceModeReplicated>>,
}
