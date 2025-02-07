#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryTaskSourceTriggerAuthentication {
    /// Time in seconds that the token remains valid.
    #[builder(into, default)]
    #[serde(rename = "expireInSeconds")]
    pub r#expire_in_seconds: Box<Option<i32>>,
    /// The refresh token used to refresh the access token.
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<String>>,
    /// The scope of the access token.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// The access token used to access the source control provider.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: Box<String>,
    /// The type of the token. Possible values are `PAT` (personal access token) and `OAuth`.
    #[builder(into)]
    #[serde(rename = "tokenType")]
    pub r#token_type: Box<String>,
}
