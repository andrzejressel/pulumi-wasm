#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorCapacityProvisionedCapacity {
    /// The number of microcontroller units (MCUs) allocated to each connector worker. Valid values: `1`, `2`, `4`, `8`. The default value is `1`.
    #[builder(into, default)]
    #[serde(rename = "mcuCount")]
    pub r#mcu_count: Box<Option<i32>>,
    /// The number of workers that are allocated to the connector.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Box<i32>,
}
