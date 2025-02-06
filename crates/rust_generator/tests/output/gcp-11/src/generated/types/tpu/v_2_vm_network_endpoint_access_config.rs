#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2VmNetworkEndpointAccessConfig {
    /// (Output)
    /// An external IP address associated with the TPU worker.
    #[builder(into, default)]
    #[serde(rename = "externalIp")]
    pub r#external_ip: Box<Option<String>>,
}
