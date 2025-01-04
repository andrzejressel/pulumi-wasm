#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeviceDeviceProperty {
    /// The Data Box Edge/Gateway device local capacity in MB.
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<i32>>,
    /// Type of compute roles configured.
    #[builder(into, default)]
    #[serde(rename = "configuredRoleTypes")]
    pub r#configured_role_types: Box<Option<Vec<String>>>,
    /// The Data Box Edge/Gateway device culture.
    #[builder(into, default)]
    #[serde(rename = "culture")]
    pub r#culture: Box<Option<String>>,
    /// The device software version number of the device (e.g. 1.2.18105.6).
    #[builder(into, default)]
    #[serde(rename = "hcsVersion")]
    pub r#hcs_version: Box<Option<String>>,
    /// The Data Box Edge/Gateway device model.
    #[builder(into, default)]
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    /// The number of nodes in the cluster.
    #[builder(into, default)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<Option<i32>>,
    /// The Serial Number of Data Box Edge/Gateway device.
    #[builder(into, default)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    /// The Data Box Edge/Gateway device software version.
    #[builder(into, default)]
    #[serde(rename = "softwareVersion")]
    pub r#software_version: Box<Option<String>>,
    /// The status of the Data Box Edge/Gateway device.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// The Data Box Edge/Gateway device timezone.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
    /// The type of the Data Box Edge/Gateway device.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
