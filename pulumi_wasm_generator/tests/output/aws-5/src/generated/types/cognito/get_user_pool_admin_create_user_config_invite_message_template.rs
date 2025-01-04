#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPoolAdminCreateUserConfigInviteMessageTemplate {
    /// - Email message content.
    #[builder(into)]
    #[serde(rename = "emailMessage")]
    pub r#email_message: Box<String>,
    /// - Email message subject.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Box<String>,
    /// - SMS message content.
    #[builder(into)]
    #[serde(rename = "smsMessage")]
    pub r#sms_message: Box<String>,
}
