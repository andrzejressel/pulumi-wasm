#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppCampaignHook {
    /// Lambda function name or ARN to be called for delivery. Conflicts with `web_url`
    #[builder(into, default)]
    #[serde(rename = "lambdaFunctionName")]
    pub r#lambda_function_name: Box<Option<String>>,
    /// What mode Lambda should be invoked in. Valid values for this parameter are `DELIVERY`, `FILTER`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Web URL to call for hook. If the URL has authentication specified it will be added as authentication to the request. Conflicts with `lambda_function_name`
    #[builder(into, default)]
    #[serde(rename = "webUrl")]
    pub r#web_url: Box<Option<String>>,
}
