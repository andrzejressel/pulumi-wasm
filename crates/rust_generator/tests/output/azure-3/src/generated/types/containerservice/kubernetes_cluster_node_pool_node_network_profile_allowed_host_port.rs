#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterNodePoolNodeNetworkProfileAllowedHostPort {
    /// Specifies the end of the port range.
    #[builder(into, default)]
    #[serde(rename = "portEnd")]
    pub r#port_end: Box<Option<i32>>,
    /// Specifies the start of the port range.
    #[builder(into, default)]
    #[serde(rename = "portStart")]
    pub r#port_start: Box<Option<i32>>,
    /// Specifies the protocol of the port range. Possible values are `TCP` and `UDP`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
