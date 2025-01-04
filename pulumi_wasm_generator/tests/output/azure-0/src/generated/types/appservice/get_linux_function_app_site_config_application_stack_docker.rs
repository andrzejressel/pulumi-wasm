#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxFunctionAppSiteConfigApplicationStackDocker {
    /// The name of the Docker image used.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<String>,
    /// The image tag of the image used.
    #[builder(into)]
    #[serde(rename = "imageTag")]
    pub r#image_tag: Box<String>,
    /// The password for the account to use to connect to the registry.
    #[builder(into)]
    #[serde(rename = "registryPassword")]
    pub r#registry_password: Box<String>,
    /// The URL of the docker registry.
    #[builder(into)]
    #[serde(rename = "registryUrl")]
    pub r#registry_url: Box<String>,
    /// The username used for connections to the registry.
    #[builder(into)]
    #[serde(rename = "registryUsername")]
    pub r#registry_username: Box<String>,
}
