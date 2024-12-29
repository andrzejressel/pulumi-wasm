/// Manages an EKS Cluster.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .vpc_config(
///                 ClusterVpcConfig::builder()
///                     .subnetIds(vec!["${example1.id}", "${example2.id}",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example IAM Role for EKS Cluster
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: eks-cluster-example
///       assumeRolePolicy: ${assumeRole.json}
///   example-AmazonEKSClusterPolicy:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSClusterPolicy
///       role: ${example.name}
///   # Optionally, enable Security Groups for Pods
///   # Reference: https://docs.aws.amazon.com/eks/latest/userguide/security-groups-for-pods.html
///   example-AmazonEKSVPCResourceController:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSVPCResourceController
///       role: ${example.name}
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - eks.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Enabling Control Plane Logging
///
/// [EKS Control Plane Logging](https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html) can be enabled via the `enabled_cluster_log_types` argument. To manage the CloudWatch Log Group retention period, the `aws.cloudwatch.LogGroup` resource can be used.
///
/// > The below configuration uses [`dependsOn`](https://www.pulumi.com/docs/intro/concepts/programming-model/#dependson) to prevent ordering issues with EKS automatically creating the log group first and a variable for naming consistency. Other ordering and naming methodologies may be more appropriate for your environment.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .enabled_cluster_log_types(vec!["api", "audit",])
///             .name("${clusterName}")
///             .build_struct(),
///     );
///     let exampleLogGroup = log_group::create(
///         "exampleLogGroup",
///         LogGroupArgs::builder()
///             .name("/aws/eks/${clusterName}/cluster")
///             .retention_in_days(7)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enabling IAM Roles for Service Accounts
///
/// For more information about this feature, see the [EKS User Guide](https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html).
///
/// ```yaml
/// resources:
///   exampleCluster:
///     type: aws:eks:Cluster
///     name: example
///   exampleOpenIdConnectProvider:
///     type: aws:iam:OpenIdConnectProvider
///     name: example
///     properties:
///       clientIdLists:
///         - sts.amazonaws.com
///       thumbprintLists:
///         - ${example.certificates[0].sha1Fingerprint}
///       url: ${example.url}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       assumeRolePolicy: ${exampleAssumeRolePolicy.json}
///       name: example
/// variables:
///   example:
///     fn::invoke:
///       Function: tls:getCertificate
///       Arguments:
///         url: ${exampleCluster.identities[0].oidcs[0].issuer}
///   exampleAssumeRolePolicy:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - sts:AssumeRoleWithWebIdentity
///             effect: Allow
///             conditions:
///               - test: StringEquals
///                 variable:
///                   fn::join:
///                     -
///                     - - fn::invoke:
///                           Function: std:replace
///                           Arguments:
///                             text: ${exampleOpenIdConnectProvider.url}
///                             search: https://
///                             replace:
///                           Return: result
///                       - :sub
///                 values:
///                   - system:serviceaccount:kube-system:aws-node
///             principals:
///               - identifiers:
///                   - ${exampleOpenIdConnectProvider.arn}
///                 type: Federated
/// ```
///
/// ### EKS Cluster on AWS Outpost
///
/// [Creating a local Amazon EKS cluster on an AWS Outpost](https://docs.aws.amazon.com/eks/latest/userguide/create-cluster-outpost.html)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${exampleAssumeRolePolicy.json}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .name("example-cluster")
///             .outpost_config(
///                 ClusterOutpostConfig::builder()
///                     .controlPlaneInstanceType("m5d.large")
///                     .outpostArns(vec!["${exampleAwsOutpostsOutpost.arn}",])
///                     .build_struct(),
///             )
///             .role_arn("${example.arn}")
///             .vpc_config(
///                 ClusterVpcConfig::builder()
///                     .endpointPrivateAccess(true)
///                     .endpointPublicAccess(false)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### EKS Cluster with Access Config
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${exampleAssumeRolePolicy.json}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .access_config(
///                 ClusterAccessConfig::builder()
///                     .authenticationMode("CONFIG_MAP")
///                     .bootstrapClusterCreatorAdminPermissions(true)
///                     .build_struct(),
///             )
///             .name("example-cluster")
///             .role_arn("${example.arn}")
///             .vpc_config(
///                 ClusterVpcConfig::builder()
///                     .endpointPrivateAccess(true)
///                     .endpointPublicAccess(false)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// After adding inline IAM Policies (e.g., `aws.iam.RolePolicy` resource) or attaching IAM Policies (e.g., `aws.iam.Policy` resource and `aws.iam.RolePolicyAttachment` resource) with the desired permissions to the IAM Role, annotate the Kubernetes service account (e.g., `kubernetes_service_account` resource) and recreate any pods.
///
/// ## Import
///
/// Using `pulumi import`, import EKS Clusters using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:eks/cluster:Cluster my_cluster my_cluster
/// ```
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Configuration block for the access config associated with your cluster, see [Amazon EKS Access Entries](https://docs.aws.amazon.com/eks/latest/userguide/access-entries.html).
        #[builder(into, default)]
        pub access_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterAccessConfig>,
        >,
        /// Install default unmanaged add-ons, such as `aws-cni`, `kube-proxy`, and CoreDNS during cluster creation. If `false`, you must manually install desired add-ons. Changing this value will force a new cluster to be created. Defaults to `true`.
        #[builder(into, default)]
        pub bootstrap_self_managed_addons: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub default_addons_to_removes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of the desired control plane logging to enable. For more information, see [Amazon EKS Control Plane Logging](https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html).
        #[builder(into, default)]
        pub enabled_cluster_log_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block with encryption configuration for the cluster. Detailed below.
        #[builder(into, default)]
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterEncryptionConfig>,
        >,
        /// Configuration block with kubernetes network configuration for the cluster. Detailed below. If removed, this provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub kubernetes_network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterKubernetesNetworkConfig>,
        >,
        /// Name of the cluster. Must be between 1-100 characters in length. Must begin with an alphanumeric character, and must only contain alphanumeric characters, dashes and underscores (`^[0-9A-Za-z][A-Za-z0-9\-_]*$`).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block representing the configuration of your local Amazon EKS cluster on an AWS Outpost. This block isn't available for creating Amazon EKS clusters on the AWS cloud.
        #[builder(into, default)]
        pub outpost_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterOutpostConfig>,
        >,
        /// ARN of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf. Ensure the resource configuration includes explicit dependencies on the IAM Role permissions by adding `depends_on` if using the `aws.iam.RolePolicy` resource or `aws.iam.RolePolicyAttachment` resource, otherwise EKS cannot delete EKS managed EC2 infrastructure such as Security Groups on EKS Cluster deletion.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the support policy to use for the cluster.  See upgrade_policy for details.
        #[builder(into, default)]
        pub upgrade_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterUpgradePolicy>,
        >,
        /// Desired Kubernetes master version. If you do not specify a value, the latest available version at resource creation is used and no upgrades will occur except those automatically triggered by EKS. The value must be configured and increased to upgrade the version when desired. Downgrades are not supported by EKS.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the VPC associated with your cluster. Amazon EKS VPC resources have specific requirements to work properly with Kubernetes. For more information, see [Cluster VPC Considerations](https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html) and [Cluster Security Group Considerations](https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html) in the Amazon EKS User Guide. Detailed below. Also contains attributes detailed in the Attributes section.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_config: pulumi_wasm_rust::Output<
            super::super::types::eks::ClusterVpcConfig,
        >,
        /// Configuration block with zonal shift configuration for the cluster. Detailed below.
        #[builder(into, default)]
        pub zonal_shift_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterZonalShiftConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Configuration block for the access config associated with your cluster, see [Amazon EKS Access Entries](https://docs.aws.amazon.com/eks/latest/userguide/access-entries.html).
        pub access_config: pulumi_wasm_rust::Output<
            super::super::types::eks::ClusterAccessConfig,
        >,
        /// ARN of the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Install default unmanaged add-ons, such as `aws-cni`, `kube-proxy`, and CoreDNS during cluster creation. If `false`, you must manually install desired add-ons. Changing this value will force a new cluster to be created. Defaults to `true`.
        pub bootstrap_self_managed_addons: pulumi_wasm_rust::Output<Option<bool>>,
        pub certificate_authorities: pulumi_wasm_rust::Output<
            Vec<super::super::types::eks::ClusterCertificateAuthority>,
        >,
        /// Attribute block containing `certificate-authority-data` for your cluster. Detailed below.
        pub certificate_authority: pulumi_wasm_rust::Output<
            super::super::types::eks::ClusterCertificateAuthority,
        >,
        /// The ID of your local Amazon EKS cluster on the AWS Outpost. This attribute isn't available for an AWS EKS cluster on AWS cloud.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Unix epoch timestamp in seconds for when the cluster was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        pub default_addons_to_removes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of the desired control plane logging to enable. For more information, see [Amazon EKS Control Plane Logging](https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html).
        pub enabled_cluster_log_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block with encryption configuration for the cluster. Detailed below.
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterEncryptionConfig>,
        >,
        /// Endpoint for your Kubernetes API server.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Attribute block containing identity provider information for your cluster. Only available on Kubernetes version 1.13 and 1.14 clusters created or upgraded on or after September 3, 2019. Detailed below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::types::eks::ClusterIdentity>,
        >,
        /// Configuration block with kubernetes network configuration for the cluster. Detailed below. If removed, this provider will only perform drift detection if a configuration value is provided.
        pub kubernetes_network_config: pulumi_wasm_rust::Output<
            super::super::types::eks::ClusterKubernetesNetworkConfig,
        >,
        /// Name of the cluster. Must be between 1-100 characters in length. Must begin with an alphanumeric character, and must only contain alphanumeric characters, dashes and underscores (`^[0-9A-Za-z][A-Za-z0-9\-_]*$`).
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration block representing the configuration of your local Amazon EKS cluster on an AWS Outpost. This block isn't available for creating Amazon EKS clusters on the AWS cloud.
        pub outpost_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterOutpostConfig>,
        >,
        /// Platform version for the cluster.
        pub platform_version: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf. Ensure the resource configuration includes explicit dependencies on the IAM Role permissions by adding `depends_on` if using the `aws.iam.RolePolicy` resource or `aws.iam.RolePolicyAttachment` resource, otherwise EKS cannot delete EKS managed EC2 infrastructure such as Security Groups on EKS Cluster deletion.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Status of the EKS cluster. One of `CREATING`, `ACTIVE`, `DELETING`, `FAILED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the support policy to use for the cluster.  See upgrade_policy for details.
        pub upgrade_policy: pulumi_wasm_rust::Output<
            super::super::types::eks::ClusterUpgradePolicy,
        >,
        /// Desired Kubernetes master version. If you do not specify a value, the latest available version at resource creation is used and no upgrades will occur except those automatically triggered by EKS. The value must be configured and increased to upgrade the version when desired. Downgrades are not supported by EKS.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the VPC associated with your cluster. Amazon EKS VPC resources have specific requirements to work properly with Kubernetes. For more information, see [Cluster VPC Considerations](https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html) and [Cluster Security Group Considerations](https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html) in the Amazon EKS User Guide. Detailed below. Also contains attributes detailed in the Attributes section.
        ///
        /// The following arguments are optional:
        pub vpc_config: pulumi_wasm_rust::Output<
            super::super::types::eks::ClusterVpcConfig,
        >,
        /// Configuration block with zonal shift configuration for the cluster. Detailed below.
        pub zonal_shift_config: pulumi_wasm_rust::Output<
            Option<super::super::types::eks::ClusterZonalShiftConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_config_binding = args.access_config.get_inner();
        let bootstrap_self_managed_addons_binding = args
            .bootstrap_self_managed_addons
            .get_inner();
        let default_addons_to_removes_binding = args
            .default_addons_to_removes
            .get_inner();
        let enabled_cluster_log_types_binding = args
            .enabled_cluster_log_types
            .get_inner();
        let encryption_config_binding = args.encryption_config.get_inner();
        let kubernetes_network_config_binding = args
            .kubernetes_network_config
            .get_inner();
        let name_binding = args.name.get_inner();
        let outpost_config_binding = args.outpost_config.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let upgrade_policy_binding = args.upgrade_policy.get_inner();
        let version_binding = args.version.get_inner();
        let vpc_config_binding = args.vpc_config.get_inner();
        let zonal_shift_config_binding = args.zonal_shift_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessConfig".into(),
                    value: &access_config_binding,
                },
                register_interface::ObjectField {
                    name: "bootstrapSelfManagedAddons".into(),
                    value: &bootstrap_self_managed_addons_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAddonsToRemoves".into(),
                    value: &default_addons_to_removes_binding,
                },
                register_interface::ObjectField {
                    name: "enabledClusterLogTypes".into(),
                    value: &enabled_cluster_log_types_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesNetworkConfig".into(),
                    value: &kubernetes_network_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outpostConfig".into(),
                    value: &outpost_config_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "upgradePolicy".into(),
                    value: &upgrade_policy_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding,
                },
                register_interface::ObjectField {
                    name: "zonalShiftConfig".into(),
                    value: &zonal_shift_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessConfig".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapSelfManagedAddons".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthorities".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthority".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "defaultAddonsToRemoves".into(),
                },
                register_interface::ResultField {
                    name: "enabledClusterLogTypes".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfig".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesNetworkConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outpostConfig".into(),
                },
                register_interface::ResultField {
                    name: "platformVersion".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "upgradePolicy".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfig".into(),
                },
                register_interface::ResultField {
                    name: "zonalShiftConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            access_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessConfig").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bootstrap_self_managed_addons: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapSelfManagedAddons").unwrap(),
            ),
            certificate_authorities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthorities").unwrap(),
            ),
            certificate_authority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthority").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            default_addons_to_removes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAddonsToRemoves").unwrap(),
            ),
            enabled_cluster_log_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledClusterLogTypes").unwrap(),
            ),
            encryption_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfig").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            kubernetes_network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesNetworkConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outpost_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostConfig").unwrap(),
            ),
            platform_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformVersion").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            upgrade_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upgradePolicy").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            vpc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfig").unwrap(),
            ),
            zonal_shift_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zonalShiftConfig").unwrap(),
            ),
        }
    }
}
