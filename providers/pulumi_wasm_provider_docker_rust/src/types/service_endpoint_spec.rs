#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceEndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if 'vip' resolution mode is used
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<crate::types::ServiceEndpointSpecPort>>>,
}
