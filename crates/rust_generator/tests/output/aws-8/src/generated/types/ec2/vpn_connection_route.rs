#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnConnectionRoute {
    /// The CIDR block associated with the local subnet of the customer data center.
    #[builder(into, default)]
    #[serde(rename = "destinationCidrBlock")]
    pub r#destination_cidr_block: Box<Option<String>>,
    /// Indicates how the routes were provided.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// The current state of the static route.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
