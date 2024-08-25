#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceEndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if 'vip' resolution mode is used
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<crate::types::ServiceEndpointSpecPort>>>,
}
