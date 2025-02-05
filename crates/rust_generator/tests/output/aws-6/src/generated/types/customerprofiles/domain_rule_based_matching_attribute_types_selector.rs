#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainRuleBasedMatchingAttributeTypesSelector {
    /// The `Address` type. You can choose from `Address`, `BusinessAddress`, `MaillingAddress`, and `ShippingAddress`.
    #[builder(into, default)]
    #[serde(rename = "addresses")]
    pub r#addresses: Box<Option<Vec<String>>>,
    /// Configures the `AttributeMatchingModel`, you can either choose `ONE_TO_ONE` or `MANY_TO_MANY`.
    #[builder(into)]
    #[serde(rename = "attributeMatchingModel")]
    pub r#attribute_matching_model: Box<String>,
    /// The `Email` type. You can choose from `EmailAddress`, `BusinessEmailAddress` and `PersonalEmailAddress`.
    #[builder(into, default)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    /// The `PhoneNumber` type. You can choose from `PhoneNumber`, `HomePhoneNumber`, and `MobilePhoneNumber`.
    #[builder(into, default)]
    #[serde(rename = "phoneNumbers")]
    pub r#phone_numbers: Box<Option<Vec<String>>>,
}
