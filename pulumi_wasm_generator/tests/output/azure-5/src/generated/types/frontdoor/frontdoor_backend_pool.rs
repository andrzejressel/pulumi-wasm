#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorBackendPool {
    /// A `backend` block as defined below.
    #[builder(into)]
    #[serde(rename = "backends")]
    pub r#backends: Box<Vec<super::super::types::frontdoor::FrontdoorBackendPoolBackend>>,
    /// Specifies the name of the `backend_pool_health_probe` block within this resource to use for this `Backend Pool`.
    #[builder(into)]
    #[serde(rename = "healthProbeName")]
    pub r#health_probe_name: Box<String>,
    /// The ID of the FrontDoor.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the name of the `backend_pool_load_balancing` block within this resource to use for this `Backend Pool`.
    #[builder(into)]
    #[serde(rename = "loadBalancingName")]
    pub r#load_balancing_name: Box<String>,
    /// Specifies the name of the Backend Pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
