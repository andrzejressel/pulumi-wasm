#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceSettingsPasswordValidationPolicy {
    /// Checks if the password is a combination of lowercase, uppercase, numeric, and non-alphanumeric characters.
    #[builder(into, default)]
    #[serde(rename = "complexity")]
    pub r#complexity: Box<Option<String>>,
    /// Prevents the use of the username in the password.
    #[builder(into, default)]
    #[serde(rename = "disallowUsernameSubstring")]
    pub r#disallow_username_substring: Box<Option<bool>>,
    /// Enables or disable the password validation policy.
    #[builder(into)]
    #[serde(rename = "enablePasswordPolicy")]
    pub r#enable_password_policy: Box<bool>,
    /// Specifies the minimum number of characters that the password must have.
    #[builder(into, default)]
    #[serde(rename = "minLength")]
    pub r#min_length: Box<Option<i32>>,
    /// Specifies the minimum duration after which you can change the password.
    #[builder(into, default)]
    #[serde(rename = "passwordChangeInterval")]
    pub r#password_change_interval: Box<Option<String>>,
    /// Specifies the number of previous passwords that you can't reuse.
    #[builder(into, default)]
    #[serde(rename = "reuseInterval")]
    pub r#reuse_interval: Box<Option<i32>>,
}
