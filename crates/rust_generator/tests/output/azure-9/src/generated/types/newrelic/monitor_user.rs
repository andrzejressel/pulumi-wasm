#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitorUser {
    /// Specifies the user Email. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// Specifies the first name. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into)]
    #[serde(rename = "firstName")]
    pub r#first_name: Box<String>,
    /// Specifies the last name. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into)]
    #[serde(rename = "lastName")]
    pub r#last_name: Box<String>,
    /// Specifies the contact phone number. Changing this forces a new Azure Native New Relic Monitor to be created.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<String>,
}
