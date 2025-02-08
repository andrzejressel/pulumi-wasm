#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDeviceDeviceProperty {
    /// The Data Box Edge/Gateway device local capacity in MB.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<i32>,
    /// Type of compute roles configured.
    #[builder(into)]
    #[serde(rename = "configuredRoleTypes")]
    pub r#configured_role_types: Box<Vec<String>>,
    /// The Data Box Edge/Gateway device culture.
    #[builder(into)]
    #[serde(rename = "culture")]
    pub r#culture: Box<String>,
    /// The device software version number of the device (e.g. 1.2.18105.6).
    #[builder(into)]
    #[serde(rename = "hcsVersion")]
    pub r#hcs_version: Box<String>,
    /// The Data Box Edge/Gateway device model.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: Box<String>,
    /// The number of nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// The Serial Number of Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<String>,
    /// The Data Box Edge/Gateway device software version.
    #[builder(into)]
    #[serde(rename = "softwareVersion")]
    pub r#software_version: Box<String>,
    /// The status of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// The Data Box Edge/Gateway device timezone.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
    /// The type of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
