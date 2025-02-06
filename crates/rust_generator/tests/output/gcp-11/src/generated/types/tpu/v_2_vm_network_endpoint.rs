#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2VmNetworkEndpoint {
    /// (Output)
    /// The access config for the TPU worker.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Box<Option<Vec<super::super::types::tpu::V2VmNetworkEndpointAccessConfig>>>,
    /// (Output)
    /// The internal IP address of this network endpoint.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// (Output)
    /// The port of this network endpoint.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
