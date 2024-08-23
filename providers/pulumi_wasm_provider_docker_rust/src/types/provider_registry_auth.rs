#[derive(serde::Serialize)]
pub struct ProviderRegistryAuth {
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
