#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TokenPasswordPassword1 {
    /// The expiration date of the password in RFC3339 format. If not specified, the password never expires. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<Option<String>>,
    /// The value of the password (Sensitive).
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}