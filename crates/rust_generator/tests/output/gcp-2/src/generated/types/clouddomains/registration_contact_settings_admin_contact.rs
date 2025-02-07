#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistrationContactSettingsAdminContact {
    /// Required. Email address of the contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// Fax number of the contact in international format. For example, "+1-800-555-0123".
    #[builder(into, default)]
    #[serde(rename = "faxNumber")]
    pub r#fax_number: Box<Option<String>>,
    /// Required. Phone number of the contact in international format. For example, "+1-800-555-0123".
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<String>,
    /// Required. Postal address of the contact.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postalAddress")]
    pub r#postal_address: Box<super::super::types::clouddomains::RegistrationContactSettingsAdminContactPostalAddress>,
}
