#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceServiceAccount {
    /// The service account e-mail address.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// A list of service scopes.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Vec<String>>,
}
