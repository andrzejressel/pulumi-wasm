#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppSiteConfigApplicationStack {
    /// The docker image, including tag, used by this Linux Web App.
    #[builder(into)]
    #[serde(rename = "dockerImageName")]
    pub r#docker_image_name: Box<String>,
    /// The User Name to use for authentication against the registry to pull the image.
    #[builder(into)]
    #[serde(rename = "dockerRegistryPassword")]
    pub r#docker_registry_password: Box<String>,
    /// The URL of the container registry where the `docker_image_name` is located.
    #[builder(into)]
    #[serde(rename = "dockerRegistryUrl")]
    pub r#docker_registry_url: Box<String>,
    /// The User Name to use for authentication against the registry to pull the image.
    #[builder(into)]
    #[serde(rename = "dockerRegistryUsername")]
    pub r#docker_registry_username: Box<String>,
    /// The version of .NET in use.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Box<String>,
    #[builder(into)]
    #[serde(rename = "goVersion")]
    pub r#go_version: Box<String>,
    /// The Java server type.
    #[builder(into)]
    #[serde(rename = "javaServer")]
    pub r#java_server: Box<String>,
    /// The Version of the `java_server` in use.
    #[builder(into)]
    #[serde(rename = "javaServerVersion")]
    pub r#java_server_version: Box<String>,
    /// The Version of Java in use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<String>,
    /// The version of Node in use.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Box<String>,
    /// The version of PHP in use.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: Box<String>,
    /// The version of Python in use.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Box<String>,
    /// The version of Ruby in use.
    #[builder(into)]
    #[serde(rename = "rubyVersion")]
    pub r#ruby_version: Box<String>,
}
