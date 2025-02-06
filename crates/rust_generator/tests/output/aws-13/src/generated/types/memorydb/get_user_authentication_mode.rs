#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserAuthenticationMode {
    /// Number of passwords belonging to the user if `type` is set to `password`.
    #[builder(into)]
    #[serde(rename = "passwordCount")]
    pub r#password_count: Box<i32>,
    /// Type of authentication configured.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
