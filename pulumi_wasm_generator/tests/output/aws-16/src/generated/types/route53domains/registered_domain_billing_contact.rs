#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegisteredDomainBillingContact {
    /// First line of the contact's address.
    #[builder(into, default)]
    #[serde(rename = "addressLine1")]
    pub r#address_line_1: Box<Option<String>>,
    /// Second line of contact's address, if any.
    #[builder(into, default)]
    #[serde(rename = "addressLine2")]
    pub r#address_line_2: Box<Option<String>>,
    /// The city of the contact's address.
    #[builder(into, default)]
    #[serde(rename = "city")]
    pub r#city: Box<Option<String>>,
    /// Indicates whether the contact is a person, company, association, or public organization. See the [AWS API documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ContactDetail.html#Route53Domains-Type-domains_ContactDetail-ContactType) for valid values.
    #[builder(into, default)]
    #[serde(rename = "contactType")]
    pub r#contact_type: Box<Option<String>>,
    /// Code for the country of the contact's address. See the [AWS API documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ContactDetail.html#Route53Domains-Type-domains_ContactDetail-CountryCode) for valid values.
    #[builder(into, default)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<Option<String>>,
    /// Email address of the contact.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// A key-value map of parameters required by certain top-level domains.
    #[builder(into, default)]
    #[serde(rename = "extraParams")]
    pub r#extra_params: Box<Option<std::collections::HashMap<String, String>>>,
    /// Fax number of the contact. Phone number must be specified in the format "+[country dialing code].[number including any area code]".
    #[builder(into, default)]
    #[serde(rename = "fax")]
    pub r#fax: Box<Option<String>>,
    /// First name of contact.
    #[builder(into, default)]
    #[serde(rename = "firstName")]
    pub r#first_name: Box<Option<String>>,
    /// Last name of contact.
    #[builder(into, default)]
    #[serde(rename = "lastName")]
    pub r#last_name: Box<Option<String>>,
    /// Name of the organization for contact types other than `PERSON`.
    #[builder(into, default)]
    #[serde(rename = "organizationName")]
    pub r#organization_name: Box<Option<String>>,
    /// The phone number of the contact. Phone number must be specified in the format "+[country dialing code].[number including any area code]".
    #[builder(into, default)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<Option<String>>,
    /// The state or province of the contact's city.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The zip or postal code of the contact's address.
    #[builder(into, default)]
    #[serde(rename = "zipCode")]
    pub r#zip_code: Box<Option<String>>,
}
