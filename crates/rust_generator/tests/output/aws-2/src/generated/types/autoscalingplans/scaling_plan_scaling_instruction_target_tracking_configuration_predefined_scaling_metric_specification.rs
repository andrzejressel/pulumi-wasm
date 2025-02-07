#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScalingPlanScalingInstructionTargetTrackingConfigurationPredefinedScalingMetricSpecification {
    /// Metric type. Valid values: `ALBRequestCountPerTarget`, `ASGAverageCPUUtilization`, `ASGAverageNetworkIn`, `ASGAverageNetworkOut`, `DynamoDBReadCapacityUtilization`, `DynamoDBWriteCapacityUtilization`, `ECSServiceAverageCPUUtilization`, `ECSServiceAverageMemoryUtilization`, `EC2SpotFleetRequestAverageCPUUtilization`, `EC2SpotFleetRequestAverageNetworkIn`, `EC2SpotFleetRequestAverageNetworkOut`, `RDSReaderAverageCPUUtilization`, `RDSReaderAverageDatabaseConnections`.
    #[builder(into)]
    #[serde(rename = "predefinedScalingMetricType")]
    pub r#predefined_scaling_metric_type: Box<String>,
    /// Identifies the resource associated with the metric type.
    #[builder(into, default)]
    #[serde(rename = "resourceLabel")]
    pub r#resource_label: Box<Option<String>>,
}
