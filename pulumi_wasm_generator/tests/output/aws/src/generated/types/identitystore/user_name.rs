#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserName {
    /// The family name of the user.
    #[builder(into)]
    #[serde(rename = "familyName")]
    pub r#family_name: Box<String>,
    /// The name that is typically displayed when the name is shown for display.
    #[builder(into, default)]
    #[serde(rename = "formatted")]
    pub r#formatted: Box<Option<String>>,
    /// The given name of the user.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "givenName")]
    pub r#given_name: Box<String>,
    /// The honorific prefix of the user.
    #[builder(into, default)]
    #[serde(rename = "honorificPrefix")]
    pub r#honorific_prefix: Box<Option<String>>,
    /// The honorific suffix of the user.
    #[builder(into, default)]
    #[serde(rename = "honorificSuffix")]
    pub r#honorific_suffix: Box<Option<String>>,
    /// The middle name of the user.
    #[builder(into, default)]
    #[serde(rename = "middleName")]
    pub r#middle_name: Box<Option<String>>,
}
