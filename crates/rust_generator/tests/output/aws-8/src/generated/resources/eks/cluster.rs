/// Manages an EKS Cluster.
///
/// ## Example Usage
///
/// ### EKS Cluster
///
/// ```yaml
/// resources:
///   example:
///     type: aws:eks:Cluster
///     properties:
///       name: example
///       accessConfig:
///         authenticationMode: API
///       roleArn: ${exampleAwsIamRole.arn}
///       version: '1.31'
///       vpcConfig:
///         subnetIds:
///           - ${az1.id}
///           - ${az2.id}
///           - ${az3.id}
///     options:
///       dependsOn:
///         - ${clusterAmazonEKSClusterPolicy}
///   cluster:
///     type: aws:iam:Role
///     properties:
///       name: eks-cluster-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: eks.amazonaws.com
///   clusterAmazonEKSClusterPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSClusterPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSClusterPolicy
///       role: ${cluster.name}
/// ```
///
/// ### EKS Cluster with EKS Auto Mode
///
/// > **NOTE:** When using EKS Auto Mode `compute_config.enabled`, `kubernetes_network_config.elastic_load_balancing.enabled`, and `storage_config.block_storage.enabled` must *ALL be set to `true`. Likewise for disabling EKS Auto Mode, all three arguments must be set to `false`. Enabling EKS Auto Mode also requires that `bootstrap_self_managed_addons` is set to `false`.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:eks:Cluster
///     properties:
///       name: example
///       accessConfig:
///         authenticationMode: API
///       roleArn: ${cluster.arn}
///       version: '1.31'
///       bootstrapSelfManagedAddons: false
///       computeConfig:
///         enabled: true
///         nodePools:
///           - general-purpose
///         nodeRoleArn: ${node.arn}
///       kubernetesNetworkConfig:
///         elasticLoadBalancing:
///           enabled: true
///       storageConfig:
///         blockStorage:
///           enabled: true
///       vpcConfig:
///         endpointPrivateAccess: true
///         endpointPublicAccess: true
///         subnetIds:
///           - ${az1.id}
///           - ${az2.id}
///           - ${az3.id}
///     options:
///       dependsOn:
///         - ${clusterAmazonEKSClusterPolicy}
///         - ${clusterAmazonEKSComputePolicy}
///         - ${clusterAmazonEKSBlockStoragePolicy}
///         - ${clusterAmazonEKSLoadBalancingPolicy}
///         - ${clusterAmazonEKSNetworkingPolicy}
///   node:
///     type: aws:iam:Role
///     properties:
///       name: eks-auto-node-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: ec2.amazonaws.com
///   nodeAmazonEKSWorkerNodeMinimalPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: node_AmazonEKSWorkerNodeMinimalPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSWorkerNodeMinimalPolicy
///       role: ${node.name}
///   nodeAmazonEC2ContainerRegistryPullOnly:
///     type: aws:iam:RolePolicyAttachment
///     name: node_AmazonEC2ContainerRegistryPullOnly
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryPullOnly
///       role: ${node.name}
///   cluster:
///     type: aws:iam:Role
///     properties:
///       name: eks-cluster-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: eks.amazonaws.com
///   clusterAmazonEKSClusterPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSClusterPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSClusterPolicy
///       role: ${cluster.name}
///   clusterAmazonEKSComputePolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSComputePolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSComputePolicy
///       role: ${cluster.name}
///   clusterAmazonEKSBlockStoragePolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSBlockStoragePolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSBlockStoragePolicy
///       role: ${cluster.name}
///   clusterAmazonEKSLoadBalancingPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSLoadBalancingPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSLoadBalancingPolicy
///       role: ${cluster.name}
///   clusterAmazonEKSNetworkingPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSNetworkingPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSNetworkingPolicy
///       role: ${cluster.name}
/// ```
///
/// ### EKS Cluster with EKS Hybrid Nodes
///
/// ```yaml
/// resources:
///   example:
///     type: aws:eks:Cluster
///     properties:
///       name: example
///       accessConfig:
///         authenticationMode: API
///       roleArn: ${cluster.arn}
///       version: '1.31'
///       clusterRemoteNetworkConfig:
///         remoteNodeNetworks:
///           cidrs:
///             - 172.16.0.0/18
///         remotePodNetworks:
///           cidrs:
///             - 172.16.64.0/18
///       vpcConfig:
///         endpointPrivateAccess: true
///         endpointPublicAccess: true
///         subnetIds:
///           - ${az1.id}
///           - ${az2.id}
///           - ${az3.id}
///     options:
///       dependsOn:
///         - ${clusterAmazonEKSClusterPolicy}
///   cluster:
///     type: aws:iam:Role
///     properties:
///       name: eks-cluster-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: eks.amazonaws.com
///   clusterAmazonEKSClusterPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSClusterPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSClusterPolicy
///       role: ${cluster.name}
/// ```
///
/// ### Local EKS Cluster on AWS Outpost
///
/// [Creating a local Amazon EKS cluster on an AWS Outpost](https://docs.aws.amazon.com/eks/latest/userguide/create-cluster-outpost.html)
///
/// ```yaml
/// resources:
///   exampleCluster:
///     type: aws:eks:Cluster
///     name: example
///     properties:
///       name: example
///       accessConfig:
///         authenticationMode: CONFIG_MAP
///       roleArn: ${exampleAwsIamRole.arn}
///       version: '1.31'
///       vpcConfig:
///         endpointPrivateAccess: true
///         endpointPublicAccess: false
///         subnetIds:
///           - ${az1.id}
///           - ${az2.id}
///           - ${az3.id}
///       outpostConfig:
///         controlPlaneInstanceType: m5.large
///         outpostArns:
///           - ${example.arn}
///     options:
///       dependsOn:
///         - ${clusterAmazonEKSLocalOutpostClusterPolicy}
///   cluster:
///     type: aws:iam:Role
///     properties:
///       name: eks-cluster-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service:
///                   - eks.amazonaws.com
///                   - ec2.amazonaws.com
///   clusterAmazonEKSLocalOutpostClusterPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: cluster_AmazonEKSLocalOutpostClusterPolicy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSLocalOutpostClusterPolicy
///       role: ${cluster.name}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:outposts:getOutpost
///       arguments:
///         name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS Clusters using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:eks/cluster:Cluster my_cluster my_cluster
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Configuration block for the access config associated with your cluster, see [Amazon EKS Access Entries](https://docs.aws.amazon.com/eks/latest/userguide/access-entries.html).
        #[builder(into, default)]
        pub access_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterAccessConfig>,
        >,
        /// Install default unmanaged add-ons, such as `aws-cni`, `kube-proxy`, and CoreDNS during cluster creation. If `false`, you must manually install desired add-ons. Changing this value will force a new cluster to be created. Defaults to `true`.
        #[builder(into, default)]
        pub bootstrap_self_managed_addons: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Configuration block with compute configuration for EKS Auto Mode. Detailed below.
        #[builder(into, default)]
        pub compute_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterComputeConfig>,
        >,
        #[builder(into, default)]
        pub default_addons_to_removes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// List of the desired control plane logging to enable. For more information, see [Amazon EKS Control Plane Logging](https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html).
        #[builder(into, default)]
        pub enabled_cluster_log_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Configuration block with encryption configuration for the cluster. Detailed below.
        #[builder(into, default)]
        pub encryption_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterEncryptionConfig>,
        >,
        /// Configuration block with kubernetes network configuration for the cluster. Detailed below. If removed, this provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub kubernetes_network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterKubernetesNetworkConfig>,
        >,
        /// Name of the cluster. Must be between 1-100 characters in length. Must begin with an alphanumeric character, and must only contain alphanumeric characters, dashes and underscores (`^[0-9A-Za-z][A-Za-z0-9\-_]*$`).
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block representing the configuration of your local Amazon EKS cluster on an AWS Outpost. This block isn't available for creating Amazon EKS clusters on the AWS cloud.
        #[builder(into, default)]
        pub outpost_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterOutpostConfig>,
        >,
        /// Configuration block with remote network configuration for EKS Hybrid Nodes. Detailed below.
        #[builder(into, default)]
        pub remote_network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterRemoteNetworkConfig>,
        >,
        /// ARN of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf. Ensure the resource configuration includes explicit dependencies on the IAM Role permissions by adding `depends_on` if using the `aws.iam.RolePolicy` resource or `aws.iam.RolePolicyAttachment` resource, otherwise EKS cannot delete EKS managed EC2 infrastructure such as Security Groups on EKS Cluster deletion.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block with storage configuration for EKS Auto Mode. Detailed below.
        #[builder(into, default)]
        pub storage_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterStorageConfig>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the support policy to use for the cluster.  See upgrade_policy for details.
        #[builder(into, default)]
        pub upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterUpgradePolicy>,
        >,
        /// Desired Kubernetes master version. If you do not specify a value, the latest available version at resource creation is used and no upgrades will occur except those automatically triggered by EKS. The value must be configured and increased to upgrade the version when desired. Downgrades are not supported by EKS.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the VPC associated with your cluster. Amazon EKS VPC resources have specific requirements to work properly with Kubernetes. For more information, see [Cluster VPC Considerations](https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html) and [Cluster Security Group Considerations](https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html) in the Amazon EKS User Guide. Detailed below. Also contains attributes detailed in the Attributes section.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::eks::ClusterVpcConfig,
        >,
        /// Configuration block with zonal shift configuration for the cluster. Detailed below.
        #[builder(into, default)]
        pub zonal_shift_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::ClusterZonalShiftConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Configuration block for the access config associated with your cluster, see [Amazon EKS Access Entries](https://docs.aws.amazon.com/eks/latest/userguide/access-entries.html).
        pub access_config: pulumi_gestalt_rust::Output<
            super::super::types::eks::ClusterAccessConfig,
        >,
        /// ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Install default unmanaged add-ons, such as `aws-cni`, `kube-proxy`, and CoreDNS during cluster creation. If `false`, you must manually install desired add-ons. Changing this value will force a new cluster to be created. Defaults to `true`.
        pub bootstrap_self_managed_addons: pulumi_gestalt_rust::Output<Option<bool>>,
        pub certificate_authorities: pulumi_gestalt_rust::Output<
            Vec<super::super::types::eks::ClusterCertificateAuthority>,
        >,
        /// Attribute block containing `certificate-authority-data` for your cluster. Detailed below.
        pub certificate_authority: pulumi_gestalt_rust::Output<
            super::super::types::eks::ClusterCertificateAuthority,
        >,
        /// The ID of your local Amazon EKS cluster on the AWS Outpost. This attribute isn't available for an AWS EKS cluster on AWS cloud.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with compute configuration for EKS Auto Mode. Detailed below.
        pub compute_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::ClusterComputeConfig>,
        >,
        /// Unix epoch timestamp in seconds for when the cluster was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        pub default_addons_to_removes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of the desired control plane logging to enable. For more information, see [Amazon EKS Control Plane Logging](https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html).
        pub enabled_cluster_log_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Configuration block with encryption configuration for the cluster. Detailed below.
        pub encryption_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::ClusterEncryptionConfig>,
        >,
        /// Endpoint for your Kubernetes API server.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Attribute block containing identity provider information for your cluster. Only available on Kubernetes version 1.13 and 1.14 clusters created or upgraded on or after September 3, 2019. Detailed below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::types::eks::ClusterIdentity>,
        >,
        /// Configuration block with kubernetes network configuration for the cluster. Detailed below. If removed, this provider will only perform drift detection if a configuration value is provided.
        pub kubernetes_network_config: pulumi_gestalt_rust::Output<
            super::super::types::eks::ClusterKubernetesNetworkConfig,
        >,
        /// Name of the cluster. Must be between 1-100 characters in length. Must begin with an alphanumeric character, and must only contain alphanumeric characters, dashes and underscores (`^[0-9A-Za-z][A-Za-z0-9\-_]*$`).
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block representing the configuration of your local Amazon EKS cluster on an AWS Outpost. This block isn't available for creating Amazon EKS clusters on the AWS cloud.
        pub outpost_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::ClusterOutpostConfig>,
        >,
        /// Platform version for the cluster.
        pub platform_version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with remote network configuration for EKS Hybrid Nodes. Detailed below.
        pub remote_network_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::ClusterRemoteNetworkConfig>,
        >,
        /// ARN of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf. Ensure the resource configuration includes explicit dependencies on the IAM Role permissions by adding `depends_on` if using the `aws.iam.RolePolicy` resource or `aws.iam.RolePolicyAttachment` resource, otherwise EKS cannot delete EKS managed EC2 infrastructure such as Security Groups on EKS Cluster deletion.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Status of the EKS cluster. One of `CREATING`, `ACTIVE`, `DELETING`, `FAILED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with storage configuration for EKS Auto Mode. Detailed below.
        pub storage_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::ClusterStorageConfig>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the support policy to use for the cluster.  See upgrade_policy for details.
        pub upgrade_policy: pulumi_gestalt_rust::Output<
            super::super::types::eks::ClusterUpgradePolicy,
        >,
        /// Desired Kubernetes master version. If you do not specify a value, the latest available version at resource creation is used and no upgrades will occur except those automatically triggered by EKS. The value must be configured and increased to upgrade the version when desired. Downgrades are not supported by EKS.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the VPC associated with your cluster. Amazon EKS VPC resources have specific requirements to work properly with Kubernetes. For more information, see [Cluster VPC Considerations](https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html) and [Cluster Security Group Considerations](https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html) in the Amazon EKS User Guide. Detailed below. Also contains attributes detailed in the Attributes section.
        ///
        /// The following arguments are optional:
        pub vpc_config: pulumi_gestalt_rust::Output<
            super::super::types::eks::ClusterVpcConfig,
        >,
        /// Configuration block with zonal shift configuration for the cluster. Detailed below.
        pub zonal_shift_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::ClusterZonalShiftConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_config_binding = args.access_config.get_output(context);
        let bootstrap_self_managed_addons_binding = args
            .bootstrap_self_managed_addons
            .get_output(context);
        let compute_config_binding = args.compute_config.get_output(context);
        let default_addons_to_removes_binding = args
            .default_addons_to_removes
            .get_output(context);
        let enabled_cluster_log_types_binding = args
            .enabled_cluster_log_types
            .get_output(context);
        let encryption_config_binding = args.encryption_config.get_output(context);
        let kubernetes_network_config_binding = args
            .kubernetes_network_config
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let outpost_config_binding = args.outpost_config.get_output(context);
        let remote_network_config_binding = args
            .remote_network_config
            .get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let storage_config_binding = args.storage_config.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let upgrade_policy_binding = args.upgrade_policy.get_output(context);
        let version_binding = args.version.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let zonal_shift_config_binding = args.zonal_shift_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:eks/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessConfig".into(),
                    value: access_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootstrapSelfManagedAddons".into(),
                    value: bootstrap_self_managed_addons_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeConfig".into(),
                    value: compute_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAddonsToRemoves".into(),
                    value: default_addons_to_removes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledClusterLogTypes".into(),
                    value: enabled_cluster_log_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfig".into(),
                    value: encryption_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesNetworkConfig".into(),
                    value: kubernetes_network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outpostConfig".into(),
                    value: outpost_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteNetworkConfig".into(),
                    value: remote_network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConfig".into(),
                    value: storage_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradePolicy".into(),
                    value: upgrade_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: vpc_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zonalShiftConfig".into(),
                    value: zonal_shift_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            access_config: o.get_field("accessConfig"),
            arn: o.get_field("arn"),
            bootstrap_self_managed_addons: o.get_field("bootstrapSelfManagedAddons"),
            certificate_authorities: o.get_field("certificateAuthorities"),
            certificate_authority: o.get_field("certificateAuthority"),
            cluster_id: o.get_field("clusterId"),
            compute_config: o.get_field("computeConfig"),
            created_at: o.get_field("createdAt"),
            default_addons_to_removes: o.get_field("defaultAddonsToRemoves"),
            enabled_cluster_log_types: o.get_field("enabledClusterLogTypes"),
            encryption_config: o.get_field("encryptionConfig"),
            endpoint: o.get_field("endpoint"),
            identities: o.get_field("identities"),
            kubernetes_network_config: o.get_field("kubernetesNetworkConfig"),
            name: o.get_field("name"),
            outpost_config: o.get_field("outpostConfig"),
            platform_version: o.get_field("platformVersion"),
            remote_network_config: o.get_field("remoteNetworkConfig"),
            role_arn: o.get_field("roleArn"),
            status: o.get_field("status"),
            storage_config: o.get_field("storageConfig"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            upgrade_policy: o.get_field("upgradePolicy"),
            version: o.get_field("version"),
            vpc_config: o.get_field("vpcConfig"),
            zonal_shift_config: o.get_field("zonalShiftConfig"),
        }
    }
}
