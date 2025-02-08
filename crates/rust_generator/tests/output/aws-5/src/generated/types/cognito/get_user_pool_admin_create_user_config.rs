#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserPoolAdminCreateUserConfig {
    /// - Whether only admins can create users.
    #[builder(into)]
    #[serde(rename = "allowAdminCreateUserOnly")]
    pub r#allow_admin_create_user_only: Box<bool>,
    #[builder(into)]
    #[serde(rename = "inviteMessageTemplates")]
    pub r#invite_message_templates: Box<Vec<super::super::types::cognito::GetUserPoolAdminCreateUserConfigInviteMessageTemplate>>,
    /// - Number of days an unconfirmed user account remains valid.
    /// * invite_message_template - Templates for invitation messages.
    #[builder(into)]
    #[serde(rename = "unusedAccountValidityDays")]
    pub r#unused_account_validity_days: Box<i32>,
}
