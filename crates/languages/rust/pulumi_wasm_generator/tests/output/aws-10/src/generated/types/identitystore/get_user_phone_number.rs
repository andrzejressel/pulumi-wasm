#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPhoneNumber {
    /// When `true`, this is the primary phone number associated with the user.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<bool>,
    /// The type of phone number.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The user's phone number.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
