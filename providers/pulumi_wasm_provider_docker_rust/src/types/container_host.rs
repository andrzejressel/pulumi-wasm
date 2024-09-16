#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ContainerHost {
    /// Hostname to add
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// IP address this hostname should resolve to.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
