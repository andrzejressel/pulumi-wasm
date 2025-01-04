#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolUserAccountWindowsUserConfiguration {
    /// Specifies login mode for the user.
    #[builder(into)]
    #[serde(rename = "loginMode")]
    pub r#login_mode: Box<String>,
}
