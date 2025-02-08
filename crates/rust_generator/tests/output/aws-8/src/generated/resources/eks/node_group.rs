/// Manages an EKS Node Group, which can provision and optionally update an Auto Scaling Group of Kubernetes worker nodes compatible with EKS. Additional documentation about this functionality can be found in the [EKS User Guide](https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html).
///
/// ## Example Usage
///
///
/// ### Ignoring Changes to Desired Size
///
/// You can utilize [ignoreChanges](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) create an EKS Node Group with an initial size of running instances, then ignore any changes to that count caused externally (e.g. Application Autoscaling).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = node_group::create(
///         "example",
///         NodeGroupArgs::builder()
///             .scaling_config(
///                 NodeGroupScalingConfig::builder().desiredSize(2).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example IAM Role for EKS Node Group
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: eks-node-group-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: ec2.amazonaws.com
///           Version: 2012-10-17
///   example-AmazonEKSWorkerNodePolicy:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy
///       role: ${example.name}
///   example-AmazonEKSCNIPolicy:
///     type: aws:iam:RolePolicyAttachment
///     name: example-AmazonEKS_CNI_Policy
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy
///       role: ${example.name}
///   example-AmazonEC2ContainerRegistryReadOnly:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly
///       role: ${example.name}
/// ```
///
/// ### Example Subnets for EKS Node Group
///
///
/// ## Import
///
/// Using `pulumi import`, import EKS Node Groups using the `cluster_name` and `node_group_name` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/nodeGroup:NodeGroup my_node_group my_cluster:my_node_group
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod node_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodeGroupArgs {
        /// Type of Amazon Machine Image (AMI) associated with the EKS Node Group. See the [AWS documentation](https://docs.aws.amazon.com/eks/latest/APIReference/API_Nodegroup.html#AmazonEKS-Type-Nodegroup-amiType) for valid values. This provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub ami_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of capacity associated with the EKS Node Group. Valid values: `ON_DEMAND`, `SPOT`. This provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub capacity_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Disk size in GiB for worker nodes. Defaults to `50` for Windows, `20` all other node groups. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub disk_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Force version update if existing pods are unable to be drained due to a pod disruption budget issue.
        #[builder(into, default)]
        pub force_update_version: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of instance types associated with the EKS Node Group. Defaults to `["t3.medium"]`. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub instance_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value map of Kubernetes labels. Only labels that are applied with the EKS API are managed by this argument. Other Kubernetes labels applied to the EKS Node Group will not be managed.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block with Launch Template settings. See `launch_template` below for details. Conflicts with `remote_access`.
        #[builder(into, default)]
        pub launch_template: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::NodeGroupLaunchTemplate>,
        >,
        /// Name of the EKS Node Group. If omitted, the provider will assign a random, unique name. Conflicts with `node_group_name_prefix`. The node group name can't be longer than 63 characters. It must start with a letter or digit, but can also include hyphens and underscores for the remaining characters.
        #[builder(into, default)]
        pub node_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `node_group_name`.
        #[builder(into, default)]
        pub node_group_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the IAM Role that provides permissions for the EKS Node Group.
        #[builder(into)]
        pub node_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AMI version of the EKS Node Group. Defaults to latest version for Kubernetes version.
        #[builder(into, default)]
        pub release_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with remote access settings. See `remote_access` below for details. Conflicts with `launch_template`.
        #[builder(into, default)]
        pub remote_access: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::NodeGroupRemoteAccess>,
        >,
        /// Configuration block with scaling settings. See `scaling_config` below for details.
        #[builder(into)]
        pub scaling_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::eks::NodeGroupScalingConfig,
        >,
        /// Identifiers of EC2 Subnets to associate with the EKS Node Group.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Kubernetes taints to be applied to the nodes in the node group. Maximum of 50 taints per node group. See taint below for details.
        #[builder(into, default)]
        pub taints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::eks::NodeGroupTaint>>,
        >,
        /// Configuration block with update settings. See `update_config` below for details.
        #[builder(into, default)]
        pub update_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eks::NodeGroupUpdateConfig>,
        >,
        /// Kubernetes version. Defaults to EKS Cluster Kubernetes version. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodeGroupResult {
        /// Type of Amazon Machine Image (AMI) associated with the EKS Node Group. See the [AWS documentation](https://docs.aws.amazon.com/eks/latest/APIReference/API_Nodegroup.html#AmazonEKS-Type-Nodegroup-amiType) for valid values. This provider will only perform drift detection if a configuration value is provided.
        pub ami_type: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the EKS Node Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Type of capacity associated with the EKS Node Group. Valid values: `ON_DEMAND`, `SPOT`. This provider will only perform drift detection if a configuration value is provided.
        pub capacity_type: pulumi_gestalt_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Disk size in GiB for worker nodes. Defaults to `50` for Windows, `20` all other node groups. The provider will only perform drift detection if a configuration value is provided.
        pub disk_size: pulumi_gestalt_rust::Output<i32>,
        /// Force version update if existing pods are unable to be drained due to a pod disruption budget issue.
        pub force_update_version: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of instance types associated with the EKS Node Group. Defaults to `["t3.medium"]`. The provider will only perform drift detection if a configuration value is provided.
        pub instance_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value map of Kubernetes labels. Only labels that are applied with the EKS API are managed by this argument. Other Kubernetes labels applied to the EKS Node Group will not be managed.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block with Launch Template settings. See `launch_template` below for details. Conflicts with `remote_access`.
        pub launch_template: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::NodeGroupLaunchTemplate>,
        >,
        /// Name of the EKS Node Group. If omitted, the provider will assign a random, unique name. Conflicts with `node_group_name_prefix`. The node group name can't be longer than 63 characters. It must start with a letter or digit, but can also include hyphens and underscores for the remaining characters.
        pub node_group_name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `node_group_name`.
        pub node_group_name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role that provides permissions for the EKS Node Group.
        pub node_role_arn: pulumi_gestalt_rust::Output<String>,
        /// AMI version of the EKS Node Group. Defaults to latest version for Kubernetes version.
        pub release_version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with remote access settings. See `remote_access` below for details. Conflicts with `launch_template`.
        pub remote_access: pulumi_gestalt_rust::Output<
            Option<super::super::types::eks::NodeGroupRemoteAccess>,
        >,
        /// List of objects containing information about underlying resources.
        pub resources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::eks::NodeGroupResource>,
        >,
        /// Configuration block with scaling settings. See `scaling_config` below for details.
        pub scaling_config: pulumi_gestalt_rust::Output<
            super::super::types::eks::NodeGroupScalingConfig,
        >,
        /// Status of the EKS Node Group.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Identifiers of EC2 Subnets to associate with the EKS Node Group.
        ///
        /// The following arguments are optional:
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Kubernetes taints to be applied to the nodes in the node group. Maximum of 50 taints per node group. See taint below for details.
        pub taints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::eks::NodeGroupTaint>>,
        >,
        /// Configuration block with update settings. See `update_config` below for details.
        pub update_config: pulumi_gestalt_rust::Output<
            super::super::types::eks::NodeGroupUpdateConfig,
        >,
        /// Kubernetes version. Defaults to EKS Cluster Kubernetes version. The provider will only perform drift detection if a configuration value is provided.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NodeGroupArgs,
    ) -> NodeGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ami_type_binding = args.ami_type.get_output(context).get_inner();
        let capacity_type_binding = args.capacity_type.get_output(context).get_inner();
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let disk_size_binding = args.disk_size.get_output(context).get_inner();
        let force_update_version_binding = args
            .force_update_version
            .get_output(context)
            .get_inner();
        let instance_types_binding = args.instance_types.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let launch_template_binding = args
            .launch_template
            .get_output(context)
            .get_inner();
        let node_group_name_binding = args
            .node_group_name
            .get_output(context)
            .get_inner();
        let node_group_name_prefix_binding = args
            .node_group_name_prefix
            .get_output(context)
            .get_inner();
        let node_role_arn_binding = args.node_role_arn.get_output(context).get_inner();
        let release_version_binding = args
            .release_version
            .get_output(context)
            .get_inner();
        let remote_access_binding = args.remote_access.get_output(context).get_inner();
        let scaling_config_binding = args.scaling_config.get_output(context).get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let taints_binding = args.taints.get_output(context).get_inner();
        let update_config_binding = args.update_config.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/nodeGroup:NodeGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amiType".into(),
                    value: &ami_type_binding,
                },
                register_interface::ObjectField {
                    name: "capacityType".into(),
                    value: &capacity_type_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "diskSize".into(),
                    value: &disk_size_binding,
                },
                register_interface::ObjectField {
                    name: "forceUpdateVersion".into(),
                    value: &force_update_version_binding,
                },
                register_interface::ObjectField {
                    name: "instanceTypes".into(),
                    value: &instance_types_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "launchTemplate".into(),
                    value: &launch_template_binding,
                },
                register_interface::ObjectField {
                    name: "nodeGroupName".into(),
                    value: &node_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeGroupNamePrefix".into(),
                    value: &node_group_name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "nodeRoleArn".into(),
                    value: &node_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "releaseVersion".into(),
                    value: &release_version_binding,
                },
                register_interface::ObjectField {
                    name: "remoteAccess".into(),
                    value: &remote_access_binding,
                },
                register_interface::ObjectField {
                    name: "scalingConfig".into(),
                    value: &scaling_config_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "taints".into(),
                    value: &taints_binding,
                },
                register_interface::ObjectField {
                    name: "updateConfig".into(),
                    value: &update_config_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NodeGroupResult {
            ami_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("amiType"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            capacity_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacityType"),
            ),
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            disk_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskSize"),
            ),
            force_update_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceUpdateVersion"),
            ),
            instance_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceTypes"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            launch_template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchTemplate"),
            ),
            node_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeGroupName"),
            ),
            node_group_name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeGroupNamePrefix"),
            ),
            node_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeRoleArn"),
            ),
            release_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseVersion"),
            ),
            remote_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("remoteAccess"),
            ),
            resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resources"),
            ),
            scaling_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalingConfig"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            taints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("taints"),
            ),
            update_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateConfig"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
