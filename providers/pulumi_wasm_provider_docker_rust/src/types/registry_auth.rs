#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RegistryAuth {
    /// Address of the registry
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    /// Path to docker json file for registry auth. Defaults to `~/.docker/config.json`. If `DOCKER_CONFIG` is set, the value of `DOCKER_CONFIG` is used as the path. `config_file` has predencen over all other options.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    /// Plain content of the docker json file for registry auth. `config_file_content` has precedence over username/password.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    /// Password for the registry. Defaults to `DOCKER_REGISTRY_PASS` env variable if set.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Username for the registry. Defaults to `DOCKER_REGISTRY_USER` env variable if set.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
