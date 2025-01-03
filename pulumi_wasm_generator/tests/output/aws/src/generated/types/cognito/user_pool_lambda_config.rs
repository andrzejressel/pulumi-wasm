#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolLambdaConfig {
    /// ARN of the lambda creating an authentication challenge.
    #[builder(into, default)]
    #[serde(rename = "createAuthChallenge")]
    pub r#create_auth_challenge: Box<Option<String>>,
    /// A custom email sender AWS Lambda trigger. See custom_email_sender Below.
    #[builder(into, default)]
    #[serde(rename = "customEmailSender")]
    pub r#custom_email_sender: Box<Option<super::super::types::cognito::UserPoolLambdaConfigCustomEmailSender>>,
    /// Custom Message AWS Lambda trigger.
    #[builder(into, default)]
    #[serde(rename = "customMessage")]
    pub r#custom_message: Box<Option<String>>,
    /// A custom SMS sender AWS Lambda trigger. See custom_sms_sender Below.
    #[builder(into, default)]
    #[serde(rename = "customSmsSender")]
    pub r#custom_sms_sender: Box<Option<super::super::types::cognito::UserPoolLambdaConfigCustomSmsSender>>,
    /// Defines the authentication challenge.
    #[builder(into, default)]
    #[serde(rename = "defineAuthChallenge")]
    pub r#define_auth_challenge: Box<Option<String>>,
    /// The Amazon Resource Name of Key Management Service Customer master keys. Amazon Cognito uses the key to encrypt codes and temporary passwords sent to CustomEmailSender and CustomSMSSender.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Post-authentication AWS Lambda trigger.
    #[builder(into, default)]
    #[serde(rename = "postAuthentication")]
    pub r#post_authentication: Box<Option<String>>,
    /// Post-confirmation AWS Lambda trigger.
    #[builder(into, default)]
    #[serde(rename = "postConfirmation")]
    pub r#post_confirmation: Box<Option<String>>,
    /// Pre-authentication AWS Lambda trigger.
    #[builder(into, default)]
    #[serde(rename = "preAuthentication")]
    pub r#pre_authentication: Box<Option<String>>,
    /// Pre-registration AWS Lambda trigger.
    #[builder(into, default)]
    #[serde(rename = "preSignUp")]
    pub r#pre_sign_up: Box<Option<String>>,
    /// Allow to customize identity token claims before token generation. Set this parameter for legacy purposes; for new instances of pre token generation triggers, set the lambda_arn of `pre_token_generation_config`.
    #[builder(into, default)]
    #[serde(rename = "preTokenGeneration")]
    pub r#pre_token_generation: Box<Option<String>>,
    /// Allow to customize access tokens. See pre_token_configuration_type
    #[builder(into, default)]
    #[serde(rename = "preTokenGenerationConfig")]
    pub r#pre_token_generation_config: Box<Option<super::super::types::cognito::UserPoolLambdaConfigPreTokenGenerationConfig>>,
    /// User migration Lambda config type.
    #[builder(into, default)]
    #[serde(rename = "userMigration")]
    pub r#user_migration: Box<Option<String>>,
    /// Verifies the authentication challenge response.
    #[builder(into, default)]
    #[serde(rename = "verifyAuthChallengeResponse")]
    pub r#verify_auth_challenge_response: Box<Option<String>>,
}
