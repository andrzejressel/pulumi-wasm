#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceIpAddress {
    /// The IPv4 address assigned.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The time this IP address will be retired, in RFC
    /// 3339 format.
    #[builder(into, default)]
    #[serde(rename = "timeToRetire")]
    pub r#time_to_retire: Box<Option<String>>,
    /// The type of this IP address.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
