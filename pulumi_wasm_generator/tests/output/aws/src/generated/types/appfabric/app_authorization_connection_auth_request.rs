#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppAuthorizationConnectionAuthRequest {
    /// The authorization code returned by the application after permission is granted in the application OAuth page (after clicking on the AuthURL)..
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Box<String>,
    /// The redirect URL that is specified in the AuthURL and the application client.
    #[builder(into)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Box<String>,
}