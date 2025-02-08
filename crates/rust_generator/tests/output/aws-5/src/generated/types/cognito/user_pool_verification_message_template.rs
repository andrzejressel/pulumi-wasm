#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserPoolVerificationMessageTemplate {
    /// Default email option. Must be either `CONFIRM_WITH_CODE` or `CONFIRM_WITH_LINK`. Defaults to `CONFIRM_WITH_CODE`.
    #[builder(into, default)]
    #[serde(rename = "defaultEmailOption")]
    pub r#default_email_option: Box<Option<String>>,
    /// Email message template. Must contain the `{####}` placeholder. Conflicts with `email_verification_message` argument.
    #[builder(into, default)]
    #[serde(rename = "emailMessage")]
    pub r#email_message: Box<Option<String>>,
    /// Email message template for sending a confirmation link to the user, it must contain the `{##Click Here##}` placeholder.
    #[builder(into, default)]
    #[serde(rename = "emailMessageByLink")]
    pub r#email_message_by_link: Box<Option<String>>,
    /// Subject line for the email message template. Conflicts with `email_verification_subject` argument.
    #[builder(into, default)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Box<Option<String>>,
    /// Subject line for the email message template for sending a confirmation link to the user.
    #[builder(into, default)]
    #[serde(rename = "emailSubjectByLink")]
    pub r#email_subject_by_link: Box<Option<String>>,
    /// SMS message template. Must contain the `{####}` placeholder. Conflicts with `sms_verification_message` argument.
    #[builder(into, default)]
    #[serde(rename = "smsMessage")]
    pub r#sms_message: Box<Option<String>>,
}
