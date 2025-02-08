#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterBrokerNodeGroupInfo {
    /// The distribution of broker nodes across availability zones ([documentation](https://docs.aws.amazon.com/msk/1.0/apireference/clusters.html#clusters-model-brokerazdistribution)). Currently the only valid value is `DEFAULT`.
    #[builder(into, default)]
    #[serde(rename = "azDistribution")]
    pub r#az_distribution: Box<Option<String>>,
    /// A list of subnets to connect to in client VPC ([documentation](https://docs.aws.amazon.com/msk/1.0/apireference/clusters.html#clusters-prop-brokernodegroupinfo-clientsubnets)).
    #[builder(into)]
    #[serde(rename = "clientSubnets")]
    pub r#client_subnets: Box<Vec<String>>,
    /// Information about the cluster access configuration. See below. For security reasons, you can't turn on public access while creating an MSK cluster. However, you can update an existing cluster to make it publicly accessible. You can also create a new cluster and then update it to make it publicly accessible ([documentation](https://docs.aws.amazon.com/msk/latest/developerguide/public-access.html)).
    #[builder(into, default)]
    #[serde(rename = "connectivityInfo")]
    pub r#connectivity_info: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfo>>,
    /// Specify the instance type to use for the kafka brokersE.g., kafka.m5.large. ([Pricing info](https://aws.amazon.com/msk/pricing/))
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// A list of the security groups to associate with the elastic network interfaces to control who can communicate with the cluster.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Vec<String>>,
    /// A block that contains information about storage volumes attached to MSK broker nodes. See below.
    #[builder(into, default)]
    #[serde(rename = "storageInfo")]
    pub r#storage_info: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfo>>,
}
