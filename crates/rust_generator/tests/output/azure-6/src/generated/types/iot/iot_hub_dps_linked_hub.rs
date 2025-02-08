#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IotHubDpsLinkedHub {
    /// The weight applied to the IoT Hub. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "allocationWeight")]
    pub r#allocation_weight: Box<Option<i32>>,
    /// Determines whether to apply allocation policies to the IoT Hub. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "applyAllocationPolicy")]
    pub r#apply_allocation_policy: Box<Option<bool>>,
    /// The connection string to connect to the IoT Hub.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<String>,
    /// The IoT Hub hostname.
    #[builder(into, default)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    /// The location of the IoT hub.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
