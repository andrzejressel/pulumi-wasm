#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleServerHighAvailability {
    /// The high availability mode for the MySQL Flexible Server. Possibles values are `SameZone` and `ZoneRedundant`.
    /// 
    /// > **NOTE:** `storage[0].auto_grow_enabled` must be enabled when `high_availability` is enabled. To change the `high_availability` for a MySQL Flexible Server created with `high_availability` disabled during creation, the resource has to be recreated.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "standbyAvailabilityZone")]
    pub r#standby_availability_zone: Box<Option<String>>,
}
