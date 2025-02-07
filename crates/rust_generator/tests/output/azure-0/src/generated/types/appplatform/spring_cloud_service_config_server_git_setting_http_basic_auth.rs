#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudServiceConfigServerGitSettingHttpBasicAuth {
    /// The password used to access the Git repository server, required when the Git repository server supports HTTP Basic Authentication.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username that's used to access the Git repository server, required when the Git repository server supports HTTP Basic Authentication.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
