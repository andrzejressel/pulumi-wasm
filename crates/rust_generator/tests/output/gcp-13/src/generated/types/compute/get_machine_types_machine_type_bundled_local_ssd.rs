#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMachineTypesMachineTypeBundledLocalSsd {
    /// The default disk interface if the interface is not specified.
    #[builder(into)]
    #[serde(rename = "defaultInterface")]
    pub r#default_interface: Box<String>,
    /// The number of partitions.
    #[builder(into)]
    #[serde(rename = "partitionCount")]
    pub r#partition_count: Box<i32>,
}
