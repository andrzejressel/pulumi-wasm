#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclAssociationConfigRequestBody {
    /// Customizes the request body that your protected Amazon API Gateway REST APIs forward to AWS WAF for inspection. Applicable only when `scope` is set to `CLOUDFRONT`. See `api_gateway` below for details.
    #[builder(into, default)]
    #[serde(rename = "apiGateways")]
    pub r#api_gateways: Box<Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyApiGateway>>>,
    /// Customizes the request body that your protected Amazon App Runner services forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `app_runner_service` below for details.
    #[builder(into, default)]
    #[serde(rename = "appRunnerServices")]
    pub r#app_runner_services: Box<Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyAppRunnerService>>>,
    /// Customizes the request body that your protected Amazon CloudFront distributions forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `cloudfront` below for details.
    #[builder(into, default)]
    #[serde(rename = "cloudfronts")]
    pub r#cloudfronts: Box<Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCloudfront>>>,
    /// Customizes the request body that your protected Amazon Cognito user pools forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `cognito_user_pool` below for details.
    #[builder(into, default)]
    #[serde(rename = "cognitoUserPools")]
    pub r#cognito_user_pools: Box<Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCognitoUserPool>>>,
    /// Customizes the request body that your protected AWS Verfied Access instances forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `verified_access_instance` below for details.
    #[builder(into, default)]
    #[serde(rename = "verifiedAccessInstances")]
    pub r#verified_access_instances: Box<Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyVerifiedAccessInstance>>>,
}
