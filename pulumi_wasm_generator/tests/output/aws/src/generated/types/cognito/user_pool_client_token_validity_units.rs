#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolClientTokenValidityUnits {
    /// Time unit in for the value in `access_token_validity`, defaults to `hours`.
    #[builder(into, default)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<Option<String>>,
    /// Time unit in for the value in `id_token_validity`, defaults to `hours`.
    #[builder(into, default)]
    #[serde(rename = "idToken")]
    pub r#id_token: Box<Option<String>>,
    /// Time unit in for the value in `refresh_token_validity`, defaults to `days`.
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<String>>,
}