#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessApplicationSaasAppRefreshTokenOption {
    /// How long a refresh token will be valid for after creation. Valid units are `m`, `h` and `d`. Must be longer than 1m.
    #[builder(into, default)]
    #[serde(rename = "lifetime")]
    pub r#lifetime: Box<Option<String>>,
}
