#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecHost {
    /// The name of the host
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The ip of the host
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
