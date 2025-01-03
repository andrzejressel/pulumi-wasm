#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionApiConfig {
    /// Action to take when users access resources that require authentication.
    /// Default value is `AUTH_FAIL_ACTION_REDIRECT`.
    /// Possible values are: `AUTH_FAIL_ACTION_REDIRECT`, `AUTH_FAIL_ACTION_UNAUTHORIZED`.
    #[builder(into, default)]
    #[serde(rename = "authFailAction")]
    pub r#auth_fail_action: Box<Option<String>>,
    /// Level of login required to access this resource.
    /// Default value is `LOGIN_OPTIONAL`.
    /// Possible values are: `LOGIN_OPTIONAL`, `LOGIN_ADMIN`, `LOGIN_REQUIRED`.
    #[builder(into, default)]
    #[serde(rename = "login")]
    pub r#login: Box<Option<String>>,
    /// Path to the script from the application root directory.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Box<String>,
    /// Security (HTTPS) enforcement for this URL.
    /// Possible values are: `SECURE_DEFAULT`, `SECURE_NEVER`, `SECURE_OPTIONAL`, `SECURE_ALWAYS`.
    #[builder(into, default)]
    #[serde(rename = "securityLevel")]
    pub r#security_level: Box<Option<String>>,
    /// URL to serve the endpoint at.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}
