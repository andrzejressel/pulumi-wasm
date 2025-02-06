#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiIndexEndpointDeployedIndexDedicatedResources {
    /// The minimum number of replicas this DeployedModel will be always deployed on.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "machineSpec")]
    pub r#machine_spec: Box<super::super::types::vertex::AiIndexEndpointDeployedIndexDedicatedResourcesMachineSpec>,
    /// The maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount
    #[builder(into, default)]
    #[serde(rename = "maxReplicaCount")]
    pub r#max_replica_count: Box<Option<i32>>,
    /// The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1.
    #[builder(into)]
    #[serde(rename = "minReplicaCount")]
    pub r#min_replica_count: Box<i32>,
}
