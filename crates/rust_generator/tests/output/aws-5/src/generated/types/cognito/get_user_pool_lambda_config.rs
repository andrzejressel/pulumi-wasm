#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserPoolLambdaConfig {
    #[builder(into)]
    #[serde(rename = "createAuthChallenge")]
    pub r#create_auth_challenge: Box<String>,
    #[builder(into)]
    #[serde(rename = "customEmailSenders")]
    pub r#custom_email_senders: Box<Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomEmailSender>>,
    #[builder(into)]
    #[serde(rename = "customMessage")]
    pub r#custom_message: Box<String>,
    #[builder(into)]
    #[serde(rename = "customSmsSenders")]
    pub r#custom_sms_senders: Box<Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomSmsSender>>,
    #[builder(into)]
    #[serde(rename = "defineAuthChallenge")]
    pub r#define_auth_challenge: Box<String>,
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "postAuthentication")]
    pub r#post_authentication: Box<String>,
    #[builder(into)]
    #[serde(rename = "postConfirmation")]
    pub r#post_confirmation: Box<String>,
    #[builder(into)]
    #[serde(rename = "preAuthentication")]
    pub r#pre_authentication: Box<String>,
    #[builder(into)]
    #[serde(rename = "preSignUp")]
    pub r#pre_sign_up: Box<String>,
    #[builder(into)]
    #[serde(rename = "preTokenGeneration")]
    pub r#pre_token_generation: Box<String>,
    #[builder(into)]
    #[serde(rename = "preTokenGenerationConfigs")]
    pub r#pre_token_generation_configs: Box<Vec<super::super::types::cognito::GetUserPoolLambdaConfigPreTokenGenerationConfig>>,
    #[builder(into)]
    #[serde(rename = "userMigration")]
    pub r#user_migration: Box<String>,
    #[builder(into)]
    #[serde(rename = "verifyAuthChallengeResponse")]
    pub r#verify_auth_challenge_response: Box<String>,
}
