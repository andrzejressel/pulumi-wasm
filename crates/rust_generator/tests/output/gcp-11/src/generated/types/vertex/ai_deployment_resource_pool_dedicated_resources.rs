#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiDeploymentResourcePoolDedicatedResources {
    /// A list of the metric specifications that overrides a resource utilization metric.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "autoscalingMetricSpecs")]
    pub r#autoscaling_metric_specs: Box<Option<Vec<super::super::types::vertex::AiDeploymentResourcePoolDedicatedResourcesAutoscalingMetricSpec>>>,
    /// The specification of a single machine used by the prediction
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "machineSpec")]
    pub r#machine_spec: Box<super::super::types::vertex::AiDeploymentResourcePoolDedicatedResourcesMachineSpec>,
    /// The maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, will use min_replica_count as the default value. The value of this field impacts the charge against Vertex CPU and GPU quotas. Specifically, you will be charged for max_replica_count * number of cores in the selected machine type) and (max_replica_count * number of GPUs per replica in the selected machine type).
    #[builder(into, default)]
    #[serde(rename = "maxReplicaCount")]
    pub r#max_replica_count: Box<Option<i32>>,
    /// The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1. If traffic against the DeployedModel increases, it may dynamically be deployed onto more replicas, and as traffic decreases, some of these extra replicas may be freed.
    #[builder(into)]
    #[serde(rename = "minReplicaCount")]
    pub r#min_replica_count: Box<i32>,
}
