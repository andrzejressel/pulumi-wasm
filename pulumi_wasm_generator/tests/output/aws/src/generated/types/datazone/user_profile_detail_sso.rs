#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileDetailSso {
    #[builder(into)]
    #[serde(rename = "firstName")]
    pub r#first_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "lastName")]
    pub r#last_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<String>,
}