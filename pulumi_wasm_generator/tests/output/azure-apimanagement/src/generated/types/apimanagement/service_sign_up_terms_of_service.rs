#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSignUpTermsOfService {
    /// Should the user be asked for consent during sign up?
    #[builder(into)]
    #[serde(rename = "consentRequired")]
    pub r#consent_required: Box<bool>,
    /// Should Terms of Service be displayed during sign up?.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The Terms of Service which users are required to agree to in order to sign up.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<String>>,
}