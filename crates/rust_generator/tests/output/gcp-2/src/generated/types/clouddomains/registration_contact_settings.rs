#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistrationContactSettings {
    /// Caution: Anyone with access to this email address, phone number, and/or postal address can take control of the domain.
    /// Warning: For new Registrations, the registrant receives an email confirmation that they must complete within 15 days to
    /// avoid domain suspension.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "adminContact")]
    pub r#admin_contact: Box<super::super::types::clouddomains::RegistrationContactSettingsAdminContact>,
    /// Required. Privacy setting for the contacts associated with the Registration.
    /// Values are PUBLIC_CONTACT_DATA, PRIVATE_CONTACT_DATA, and REDACTED_CONTACT_DATA
    #[builder(into)]
    #[serde(rename = "privacy")]
    pub r#privacy: Box<String>,
    /// Caution: Anyone with access to this email address, phone number, and/or postal address can take control of the domain.
    /// Warning: For new Registrations, the registrant receives an email confirmation that they must complete within 15 days to
    /// avoid domain suspension.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "registrantContact")]
    pub r#registrant_contact: Box<super::super::types::clouddomains::RegistrationContactSettingsRegistrantContact>,
    /// Caution: Anyone with access to this email address, phone number, and/or postal address can take control of the domain.
    /// Warning: For new Registrations, the registrant receives an email confirmation that they must complete within 15 days to
    /// avoid domain suspension.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "technicalContact")]
    pub r#technical_contact: Box<super::super::types::clouddomains::RegistrationContactSettingsTechnicalContact>,
}
