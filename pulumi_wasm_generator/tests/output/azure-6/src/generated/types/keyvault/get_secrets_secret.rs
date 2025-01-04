#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecretsSecret {
    /// Whether this secret is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The ID of this secret.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of secret.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The tags of this secret.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
}
