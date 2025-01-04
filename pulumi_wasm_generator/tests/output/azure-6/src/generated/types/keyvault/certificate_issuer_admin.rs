#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateIssuerAdmin {
    /// E-mail address of the admin.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: Box<String>,
    /// First name of the admin.
    #[builder(into, default)]
    #[serde(rename = "firstName")]
    pub r#first_name: Box<Option<String>>,
    /// Last name of the admin.
    #[builder(into, default)]
    #[serde(rename = "lastName")]
    pub r#last_name: Box<Option<String>>,
    /// Phone number of the admin.
    #[builder(into, default)]
    #[serde(rename = "phone")]
    pub r#phone: Box<Option<String>>,
}
