#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderAssumeRoleWithWebIdentity {
    /// The duration, between 15 minutes and 12 hours, of the role session. Valid time units are ns, us (or µs), ms, s, h, or m.
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// IAM Policy JSON describing further restricting permissions for the IAM Role being assumed.
    #[builder(into, default)]
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
    /// Amazon Resource Names (ARNs) of IAM Policies describing further restricting permissions for the IAM Role being assumed.
    #[builder(into, default)]
    #[serde(rename = "policyArns")]
    pub r#policy_arns: Box<Option<Vec<String>>>,
    /// Amazon Resource Name (ARN) of an IAM Role to assume prior to making API calls.
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
    /// An identifier for the assumed role session.
    #[builder(into, default)]
    #[serde(rename = "sessionName")]
    pub r#session_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "webIdentityToken")]
    pub r#web_identity_token: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "webIdentityTokenFile")]
    pub r#web_identity_token_file: Box<Option<String>>,
}
