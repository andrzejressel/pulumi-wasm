#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTargetGroupStickiness {
    #[builder(into)]
    #[serde(rename = "cookieDuration")]
    pub r#cookie_duration: Box<i32>,
    #[builder(into)]
    #[serde(rename = "cookieName")]
    pub r#cookie_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
