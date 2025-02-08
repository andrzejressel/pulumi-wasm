#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolEmailConfiguration {
    /// Email configuration set name from SES.
    #[builder(into, default)]
    #[serde(rename = "configurationSet")]
    pub r#configuration_set: Box<Option<String>>,
    /// Email delivery method to use. `COGNITO_DEFAULT` for the default email functionality built into Cognito or `DEVELOPER` to use your Amazon SES configuration. Required to be `DEVELOPER` if `from_email_address` is set.
    #[builder(into, default)]
    #[serde(rename = "emailSendingAccount")]
    pub r#email_sending_account: Box<Option<String>>,
    /// Sender’s email address or sender’s display name with their email address (e.g., `john@example.com`, `John Smith <john@example.com>` or `\"John Smith Ph.D.\" <john@example.com>`). Escaped double quotes are required around display names that contain certain characters as specified in [RFC 5322](https://tools.ietf.org/html/rfc5322).
    #[builder(into, default)]
    #[serde(rename = "fromEmailAddress")]
    pub r#from_email_address: Box<Option<String>>,
    /// REPLY-TO email address.
    #[builder(into, default)]
    #[serde(rename = "replyToEmailAddress")]
    pub r#reply_to_email_address: Box<Option<String>>,
    /// ARN of the SES verified email identity to use. Required if `email_sending_account` is set to `DEVELOPER`.
    #[builder(into, default)]
    #[serde(rename = "sourceArn")]
    pub r#source_arn: Box<Option<String>>,
}
