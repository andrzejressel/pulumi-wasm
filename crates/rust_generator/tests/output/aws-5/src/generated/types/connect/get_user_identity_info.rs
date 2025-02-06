#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserIdentityInfo {
    /// The email address.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// The first name.
    #[builder(into)]
    #[serde(rename = "firstName")]
    pub r#first_name: Box<String>,
    /// The last name.
    #[builder(into)]
    #[serde(rename = "lastName")]
    pub r#last_name: Box<String>,
}
