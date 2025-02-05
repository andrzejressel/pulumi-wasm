#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolAdminCreateUserConfig {
    /// Set to True if only the administrator is allowed to create user profiles. Set to False if users can sign themselves up via an app.
    #[builder(into, default)]
    #[serde(rename = "allowAdminCreateUserOnly")]
    pub r#allow_admin_create_user_only: Box<Option<bool>>,
    /// Invite message template structure. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "inviteMessageTemplate")]
    pub r#invite_message_template: Box<Option<super::super::types::cognito::UserPoolAdminCreateUserConfigInviteMessageTemplate>>,
}
