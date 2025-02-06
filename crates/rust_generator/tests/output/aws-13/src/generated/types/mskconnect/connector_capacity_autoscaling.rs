#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorCapacityAutoscaling {
    /// The maximum number of workers allocated to the connector.
    #[builder(into)]
    #[serde(rename = "maxWorkerCount")]
    pub r#max_worker_count: Box<i32>,
    /// The number of microcontroller units (MCUs) allocated to each connector worker. Valid values: `1`, `2`, `4`, `8`. The default value is `1`.
    #[builder(into, default)]
    #[serde(rename = "mcuCount")]
    pub r#mcu_count: Box<Option<i32>>,
    /// The minimum number of workers allocated to the connector.
    #[builder(into)]
    #[serde(rename = "minWorkerCount")]
    pub r#min_worker_count: Box<i32>,
    /// The scale-in policy for the connector. See `scale_in_policy` Block for details.
    #[builder(into, default)]
    #[serde(rename = "scaleInPolicy")]
    pub r#scale_in_policy: Box<Option<super::super::types::mskconnect::ConnectorCapacityAutoscalingScaleInPolicy>>,
    /// The scale-out policy for the connector. See `scale_out_policy` Block for details.
    #[builder(into, default)]
    #[serde(rename = "scaleOutPolicy")]
    pub r#scale_out_policy: Box<Option<super::super::types::mskconnect::ConnectorCapacityAutoscalingScaleOutPolicy>>,
}
