#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactProfileLinkChannelEndPoint {
    /// Name of an end point.
    #[builder(into)]
    #[serde(rename = "endPointName")]
    pub r#end_point_name: Box<String>,
    /// IP address of an end point.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// TCP port to listen on to receive data.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<String>,
    /// Protocol of an end point. Possible values are `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}