#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceConvergeConfig {
    /// The interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// The timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
