#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInputSource {
    #[builder(into)]
    #[serde(rename = "passwordParam")]
    pub r#password_param: Box<String>,
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}