#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionInstallationState {
    /// (Output)
    /// Output only. Link to follow for next action. Empty string if the installation is already complete.
    #[builder(into, default)]
    #[serde(rename = "actionUri")]
    pub r#action_uri: Box<Option<String>>,
    /// (Output)
    /// Output only. Message of what the user should do next to continue the installation. Empty string if the installation is already complete.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// (Output)
    /// Output only. Current step of the installation process.
    #[builder(into, default)]
    #[serde(rename = "stage")]
    pub r#stage: Box<Option<String>>,
}
