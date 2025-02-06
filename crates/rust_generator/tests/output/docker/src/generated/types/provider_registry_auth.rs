#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderRegistryAuth {
    /// Address of the registry
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    /// Path to docker json file for registry auth. Defaults to `~/.docker/config.json`. If `DOCKER_CONFIG` is set, the value of `DOCKER_CONFIG` is used as the path. `config_file` has predencen over all other options.
    #[builder(into, default)]
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    /// Plain content of the docker json file for registry auth. `config_file_content` has precedence over username/password.
    #[builder(into, default)]
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    /// Password for the registry. Defaults to `DOCKER_REGISTRY_PASS` env variable if set.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Username for the registry. Defaults to `DOCKER_REGISTRY_USER` env variable if set.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
