#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetActionGroupVoiceReceiver {
    /// The country code of the voice receiver.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<String>,
    /// Specifies the name of the Action Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The phone number of the voice receiver.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<String>,
}