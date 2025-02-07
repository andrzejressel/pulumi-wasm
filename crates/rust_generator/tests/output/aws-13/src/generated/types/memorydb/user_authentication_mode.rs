#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserAuthenticationMode {
    /// Number of passwords belonging to the user if `type` is set to `password`.
    #[builder(into, default)]
    #[serde(rename = "passwordCount")]
    pub r#password_count: Box<Option<i32>>,
    /// Set of passwords used for authentication if `type` is set to `password`. You can create up to two passwords for each user.
    #[builder(into, default)]
    #[serde(rename = "passwords")]
    pub r#passwords: Box<Option<Vec<String>>>,
    /// Specifies the authentication type. Valid values are: `password` or `iam`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
