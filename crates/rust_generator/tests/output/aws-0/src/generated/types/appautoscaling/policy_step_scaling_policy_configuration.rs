#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyStepScalingPolicyConfiguration {
    /// Whether the adjustment is an absolute number or a percentage of the current capacity. Valid values are `ChangeInCapacity`, `ExactCapacity`, and `PercentChangeInCapacity`.
    #[builder(into, default)]
    #[serde(rename = "adjustmentType")]
    pub r#adjustment_type: Box<Option<String>>,
    /// Amount of time, in seconds, after a scaling activity completes and before the next scaling activity can start.
    #[builder(into, default)]
    #[serde(rename = "cooldown")]
    pub r#cooldown: Box<Option<i32>>,
    /// Aggregation type for the policy's metrics. Valid values are "Minimum", "Maximum", and "Average". Without a value, AWS will treat the aggregation type as "Average".
    #[builder(into, default)]
    #[serde(rename = "metricAggregationType")]
    pub r#metric_aggregation_type: Box<Option<String>>,
    /// Minimum number to adjust your scalable dimension as a result of a scaling activity. If the adjustment type is PercentChangeInCapacity, the scaling policy changes the scalable dimension of the scalable target by this amount.
    #[builder(into, default)]
    #[serde(rename = "minAdjustmentMagnitude")]
    pub r#min_adjustment_magnitude: Box<Option<i32>>,
    /// Set of adjustments that manage scaling. These have the following structure:
    /// 
    /// ```yaml
    /// resources:
    ///   ecsPolicy:
    ///     type: aws:appautoscaling:Policy
    ///     name: ecs_policy
    ///     properties:
    ///       stepScalingPolicyConfiguration:
    ///         stepAdjustments:
    ///           - metricIntervalLowerBound: 1
    ///             metricIntervalUpperBound: 2
    ///             scalingAdjustment: -1
    ///           - metricIntervalLowerBound: 2
    ///             metricIntervalUpperBound: 3
    ///             scalingAdjustment: 1
    /// ```
    #[builder(into, default)]
    #[serde(rename = "stepAdjustments")]
    pub r#step_adjustments: Box<Option<Vec<super::super::types::appautoscaling::PolicyStepScalingPolicyConfigurationStepAdjustment>>>,
}
