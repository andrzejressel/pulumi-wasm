#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceListenerEndpoint {
    /// Specifies the DNS address of the DB instance.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.
    #[builder(into, default)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Box<Option<String>>,
    /// The port on which the DB accepts connections.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
