#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProactiveEngagementEmergencyContact {
    /// Additional notes regarding the contact.
    #[builder(into, default)]
    #[serde(rename = "contactNotes")]
    pub r#contact_notes: Box<Option<String>>,
    /// A valid email address that will be used for this contact.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: Box<String>,
    /// A phone number, starting with `+` and up to 15 digits that will be used for this contact.
    #[builder(into, default)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<Option<String>>,
}