#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigSignInPhoneNumber {
    /// Whether phone number auth is enabled for the project or not.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A map of <test phone number, fake code> that can be used for phone auth testing.
    #[builder(into, default)]
    #[serde(rename = "testPhoneNumbers")]
    pub r#test_phone_numbers: Box<Option<std::collections::HashMap<String, String>>>,
}
