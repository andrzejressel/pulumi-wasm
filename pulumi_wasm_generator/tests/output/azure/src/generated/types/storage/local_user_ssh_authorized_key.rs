#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LocalUserSshAuthorizedKey {
    /// The description of this SSH authorized key.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The public key value of this SSH authorized key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}