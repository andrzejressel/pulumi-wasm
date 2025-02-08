#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GraphQlApiUserPoolConfig {
    /// Regular expression for validating the incoming Amazon Cognito User Pool app client ID.
    #[builder(into, default)]
    #[serde(rename = "appIdClientRegex")]
    pub r#app_id_client_regex: Box<Option<String>>,
    /// AWS region in which the user pool was created.
    #[builder(into, default)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Box<Option<String>>,
    /// Action that you want your GraphQL API to take when a request that uses Amazon Cognito User Pool authentication doesn't match the Amazon Cognito User Pool configuration. Valid: `ALLOW` and `DENY`
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
    /// User pool ID.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: Box<String>,
}
