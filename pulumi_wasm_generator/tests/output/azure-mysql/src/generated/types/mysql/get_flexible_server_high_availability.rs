#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFlexibleServerHighAvailability {
    /// The high availability mode of the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// The availability zone of the standby Flexible Server.
    #[builder(into)]
    #[serde(rename = "standbyAvailabilityZone")]
    pub r#standby_availability_zone: Box<String>,
}
