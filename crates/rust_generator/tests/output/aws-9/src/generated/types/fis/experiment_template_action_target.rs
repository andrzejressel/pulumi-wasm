#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExperimentTemplateActionTarget {
    /// Target type. Valid values are `AutoScalingGroups` (EC2 Auto Scaling groups), `Buckets` (S3 Buckets), `Cluster` (EKS Cluster), `Clusters` (ECS Clusters), `DBInstances` (RDS DB Instances), `Instances` (EC2 Instances), `Nodegroups` (EKS Node groups), `Pods` (EKS Pods), `ReplicationGroups`(ElastiCache Redis Replication Groups), `Roles` (IAM Roles), `SpotInstances` (EC2 Spot Instances), `Subnets` (VPC Subnets), `Tables` (DynamoDB encrypted global tables), `Tasks` (ECS Tasks), `TransitGateways` (Transit gateways), `Volumes` (EBS Volumes). See the [documentation](https://docs.aws.amazon.com/fis/latest/userguide/actions.html#action-targets) for more details.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Target name, referencing a corresponding target.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
