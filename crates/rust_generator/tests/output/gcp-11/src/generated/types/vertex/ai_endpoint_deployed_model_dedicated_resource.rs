#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiEndpointDeployedModelDedicatedResource {
    /// (Output)
    /// The metric specifications that overrides a resource utilization metric (CPU utilization, accelerator's duty cycle, and so on) target value (default to 60 if not set). At most one entry is allowed per metric. If machine_spec.accelerator_count is above 0, the autoscaling will be based on both CPU utilization and accelerator's duty cycle metrics and scale up when either metrics exceeds its target value while scale down if both metrics are under their target value. The default target value is 60 for both metrics. If machine_spec.accelerator_count is 0, the autoscaling will be based on CPU utilization metric only with default target value 60 if not explicitly set. For example, in the case of Online Prediction, if you want to override target CPU utilization to 80, you should set autoscaling_metric_specs.metric_name to `aiplatform.googleapis.com/prediction/online/cpu/utilization` and autoscaling_metric_specs.target to `80`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "autoscalingMetricSpecs")]
    pub r#autoscaling_metric_specs: Box<Option<Vec<super::super::types::vertex::AiEndpointDeployedModelDedicatedResourceAutoscalingMetricSpec>>>,
    /// (Output)
    /// The specification of a single machine used by the prediction.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "machineSpecs")]
    pub r#machine_specs: Box<Option<Vec<super::super::types::vertex::AiEndpointDeployedModelDedicatedResourceMachineSpec>>>,
    /// (Output)
    /// The maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, a no upper bound for scaling under heavy traffic will be assume, though Vertex AI may be unable to scale beyond certain replica number.
    #[builder(into, default)]
    #[serde(rename = "maxReplicaCount")]
    pub r#max_replica_count: Box<Option<i32>>,
    /// (Output)
    /// The minimum number of replicas this DeployedModel will be always deployed on. If traffic against it increases, it may dynamically be deployed onto more replicas up to max_replica_count, and as traffic decreases, some of these extra replicas may be freed. If the requested value is too large, the deployment will error.
    #[builder(into, default)]
    #[serde(rename = "minReplicaCount")]
    pub r#min_replica_count: Box<Option<i32>>,
}
