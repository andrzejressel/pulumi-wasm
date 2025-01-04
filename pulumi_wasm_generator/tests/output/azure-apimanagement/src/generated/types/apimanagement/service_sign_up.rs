#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSignUp {
    /// Can users sign up on the development portal?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A `terms_of_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "termsOfService")]
    pub r#terms_of_service: Box<super::super::types::apimanagement::ServiceSignUpTermsOfService>,
}
