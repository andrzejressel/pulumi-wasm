#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPoolEmailConfiguration {
    /// - Configuration set used for sending emails.
    #[builder(into)]
    #[serde(rename = "configurationSet")]
    pub r#configuration_set: Box<String>,
    /// - Email sending account.
    #[builder(into)]
    #[serde(rename = "emailSendingAccount")]
    pub r#email_sending_account: Box<String>,
    /// - Email sender address.
    #[builder(into)]
    #[serde(rename = "from")]
    pub r#from: Box<String>,
    /// - Reply-to email address.
    #[builder(into)]
    #[serde(rename = "replyToEmailAddress")]
    pub r#reply_to_email_address: Box<String>,
    /// - Source Amazon Resource Name (ARN) for emails.
    #[builder(into)]
    #[serde(rename = "sourceArn")]
    pub r#source_arn: Box<String>,
}
