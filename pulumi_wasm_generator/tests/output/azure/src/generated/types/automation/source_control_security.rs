#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SourceControlSecurity {
    /// The refresh token of specified rpeo.
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<String>>,
    /// The access token of specified repo.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: Box<String>,
    /// Specify the token type, possible values are `PersonalAccessToken` and `Oauth`.
    #[builder(into)]
    #[serde(rename = "tokenType")]
    pub r#token_type: Box<String>,
}
