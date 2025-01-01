#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSpringCloudServiceConfigServerGitSettingHttpBasicAuth {
    /// The password used to access the HTTP Basic Authentication Git repository server.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username used to access the HTTP Basic Authentication Git repository server.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
