#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationPortalOption {
    #[builder(into, default)]
    #[serde(rename = "signInOptions")]
    pub r#sign_in_options: Box<Option<Vec<super::super::types::ssoadmin::GetApplicationPortalOptionSignInOption>>>,
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: Box<String>,
}