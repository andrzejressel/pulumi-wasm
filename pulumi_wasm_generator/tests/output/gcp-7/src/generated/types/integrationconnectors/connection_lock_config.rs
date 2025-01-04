#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionLockConfig {
    /// Indicates whether or not the connection is locked.
    #[builder(into)]
    #[serde(rename = "locked")]
    pub r#locked: Box<bool>,
    /// Describes why a connection is locked.
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
}
