#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolPasswordPolicy {
    /// Minimum length of the password policy that you have set.
    #[builder(into, default)]
    #[serde(rename = "minimumLength")]
    pub r#minimum_length: Box<Option<i32>>,
    /// Number of previous passwords that you want Amazon Cognito to restrict each user from reusing. Users can't set a password that matches any of number of previous passwords specified by this argument. A value of 0 means that password history is not enforced. Valid values are between 0 and 24.
    /// 
    /// **Note:** This argument requires advanced security features to be active in the user pool.
    #[builder(into, default)]
    #[serde(rename = "passwordHistorySize")]
    pub r#password_history_size: Box<Option<i32>>,
    /// Whether you have required users to use at least one lowercase letter in their password.
    #[builder(into, default)]
    #[serde(rename = "requireLowercase")]
    pub r#require_lowercase: Box<Option<bool>>,
    /// Whether you have required users to use at least one number in their password.
    #[builder(into, default)]
    #[serde(rename = "requireNumbers")]
    pub r#require_numbers: Box<Option<bool>>,
    /// Whether you have required users to use at least one symbol in their password.
    #[builder(into, default)]
    #[serde(rename = "requireSymbols")]
    pub r#require_symbols: Box<Option<bool>>,
    /// Whether you have required users to use at least one uppercase letter in their password.
    #[builder(into, default)]
    #[serde(rename = "requireUppercase")]
    pub r#require_uppercase: Box<Option<bool>>,
    /// In the password policy you have set, refers to the number of days a temporary password is valid. If the user does not sign-in during this time, their password will need to be reset by an administrator.
    #[builder(into, default)]
    #[serde(rename = "temporaryPasswordValidityDays")]
    pub r#temporary_password_validity_days: Box<Option<i32>>,
}
