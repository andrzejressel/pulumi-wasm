#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LinuxFunctionAppSiteConfigApplicationStackDocker {
    /// The name of the Docker image to use.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<String>,
    /// The image tag of the image to use.
    #[builder(into)]
    #[serde(rename = "imageTag")]
    pub r#image_tag: Box<String>,
    /// The password for the account to use to connect to the registry.
    /// 
    /// > **NOTE:** This value is required if `container_registry_use_managed_identity` is not set to `true`.
    #[builder(into, default)]
    #[serde(rename = "registryPassword")]
    pub r#registry_password: Box<Option<String>>,
    /// The URL of the docker registry.
    #[builder(into)]
    #[serde(rename = "registryUrl")]
    pub r#registry_url: Box<String>,
    /// The username to use for connections to the registry.
    /// 
    /// > **NOTE:** This value is required if `container_registry_use_managed_identity` is not set to `true`.
    #[builder(into, default)]
    #[serde(rename = "registryUsername")]
    pub r#registry_username: Box<Option<String>>,
}
