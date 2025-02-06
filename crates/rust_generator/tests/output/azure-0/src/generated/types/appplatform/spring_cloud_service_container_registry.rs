#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudServiceContainerRegistry {
    /// Specifies the name of the container registry.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the password of the container registry.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// Specifies the login server of the container registry.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
    /// Specifies the username of the container registry.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
