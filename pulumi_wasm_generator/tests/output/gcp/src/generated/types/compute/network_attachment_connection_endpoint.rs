#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkAttachmentConnectionEndpoint {
    /// (Output)
    /// The IPv4 address assigned to the producer instance network interface. This value will be a range in case of Serverless.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// (Output)
    /// The project id or number of the interface to which the IP was assigned.
    #[builder(into, default)]
    #[serde(rename = "projectIdOrNum")]
    pub r#project_id_or_num: Box<Option<String>>,
    /// (Output)
    /// Alias IP ranges from the same subnetwork.
    #[builder(into, default)]
    #[serde(rename = "secondaryIpCidrRanges")]
    pub r#secondary_ip_cidr_ranges: Box<Option<String>>,
    /// (Output)
    /// The status of a connected endpoint to this network attachment.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// (Output)
    /// The subnetwork used to assign the IP to the producer instance network interface.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
}
