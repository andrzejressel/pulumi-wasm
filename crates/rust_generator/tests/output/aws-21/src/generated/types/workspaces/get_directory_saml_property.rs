#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDirectorySamlProperty {
    #[builder(into)]
    #[serde(rename = "relayStateParameterName")]
    pub r#relay_state_parameter_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[builder(into)]
    #[serde(rename = "userAccessUrl")]
    pub r#user_access_url: Box<String>,
}
