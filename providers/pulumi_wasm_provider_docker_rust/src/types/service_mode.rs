#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceMode {
    /// When `true`, tasks will run on every worker node. Conflicts with `replicated`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    /// The replicated service mode
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<crate::types::ServiceModeReplicated>>,
}
