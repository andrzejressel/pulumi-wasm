#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GraphQlApiAdditionalAuthenticationProvider {
    /// Authentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<String>,
    /// Nested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
    #[builder(into, default)]
    #[serde(rename = "lambdaAuthorizerConfig")]
    pub r#lambda_authorizer_config: Box<Option<super::super::types::appsync::GraphQlApiAdditionalAuthenticationProviderLambdaAuthorizerConfig>>,
    /// Nested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
    #[builder(into, default)]
    #[serde(rename = "openidConnectConfig")]
    pub r#openid_connect_config: Box<Option<super::super::types::appsync::GraphQlApiAdditionalAuthenticationProviderOpenidConnectConfig>>,
    /// Amazon Cognito User Pool configuration. See `user_pool_config` Block for details.
    #[builder(into, default)]
    #[serde(rename = "userPoolConfig")]
    pub r#user_pool_config: Box<Option<super::super::types::appsync::GraphQlApiAdditionalAuthenticationProviderUserPoolConfig>>,
}
