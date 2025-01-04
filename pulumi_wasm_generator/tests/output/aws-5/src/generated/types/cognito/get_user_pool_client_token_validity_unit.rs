#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPoolClientTokenValidityUnit {
    /// (Optional) Time unit in for the value in `access_token_validity`, defaults to `hours`.
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<String>,
    /// (Optional) Time unit in for the value in `id_token_validity`, defaults to `hours`.
    #[builder(into)]
    #[serde(rename = "idToken")]
    pub r#id_token: Box<String>,
    /// (Optional) Time unit in for the value in `refresh_token_validity`, defaults to `days`.
    #[builder(into)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<String>,
}
