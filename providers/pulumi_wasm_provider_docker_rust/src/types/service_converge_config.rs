#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceConvergeConfig {
    /// The interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`.
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// The timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
