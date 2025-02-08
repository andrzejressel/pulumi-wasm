#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserName {
    /// The family name of the user.
    #[builder(into)]
    #[serde(rename = "familyName")]
    pub r#family_name: Box<String>,
    /// The name that is typically displayed when the name is shown for display.
    #[builder(into)]
    #[serde(rename = "formatted")]
    pub r#formatted: Box<String>,
    /// The given name of the user.
    #[builder(into)]
    #[serde(rename = "givenName")]
    pub r#given_name: Box<String>,
    /// The honorific prefix of the user.
    #[builder(into)]
    #[serde(rename = "honorificPrefix")]
    pub r#honorific_prefix: Box<String>,
    /// The honorific suffix of the user.
    #[builder(into)]
    #[serde(rename = "honorificSuffix")]
    pub r#honorific_suffix: Box<String>,
    /// The middle name of the user.
    #[builder(into)]
    #[serde(rename = "middleName")]
    pub r#middle_name: Box<String>,
}
