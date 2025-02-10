#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_node_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNodeGroupArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the node group.
        #[builder(into)]
        pub node_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNodeGroupResult {
        /// Type of Amazon Machine Image (AMI) associated with the EKS Node Group.
        pub ami_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the EKS Node Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Type of capacity associated with the EKS Node Group. Valid values: `ON_DEMAND`, `SPOT`.
        pub capacity_type: pulumi_gestalt_rust::Output<String>,
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Disk size in GiB for worker nodes.
        pub disk_size: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of instance types associated with the EKS Node Group.
        pub instance_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value map of Kubernetes labels. Only labels that are applied with the EKS API are managed by this argument. Other Kubernetes labels applied to the EKS Node Group will not be managed.
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Nested attribute containing information about the launch template used to create the EKS Node Group.
        pub launch_templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupLaunchTemplate>,
        >,
        pub node_group_name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM Role that provides permissions for the EKS Node Group.
        pub node_role_arn: pulumi_gestalt_rust::Output<String>,
        /// AMI version of the EKS Node Group.
        pub release_version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with remote access settings.
        pub remote_accesses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupRemoteAccess>,
        >,
        /// List of objects containing information about underlying resources.
        pub resources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupResource>,
        >,
        /// Configuration block with scaling settings.
        pub scaling_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupScalingConfig>,
        >,
        /// Status of the EKS Node Group.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Identifiers of EC2 Subnets to associate with the EKS Node Group.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value map of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// List of objects containing information about taints applied to the nodes in the EKS Node Group.
        pub taints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupTaint>,
        >,
        /// Kubernetes version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNodeGroupArgs,
    ) -> GetNodeGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_name_binding = args.cluster_name.get_output(context);
        let node_group_name_binding = args.node_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:eks/getNodeGroup:getNodeGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: cluster_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeGroupName".into(),
                    value: node_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNodeGroupResult {
            ami_type: o.get_field("amiType"),
            arn: o.get_field("arn"),
            capacity_type: o.get_field("capacityType"),
            cluster_name: o.get_field("clusterName"),
            disk_size: o.get_field("diskSize"),
            id: o.get_field("id"),
            instance_types: o.get_field("instanceTypes"),
            labels: o.get_field("labels"),
            launch_templates: o.get_field("launchTemplates"),
            node_group_name: o.get_field("nodeGroupName"),
            node_role_arn: o.get_field("nodeRoleArn"),
            release_version: o.get_field("releaseVersion"),
            remote_accesses: o.get_field("remoteAccesses"),
            resources: o.get_field("resources"),
            scaling_configs: o.get_field("scalingConfigs"),
            status: o.get_field("status"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            taints: o.get_field("taints"),
            version: o.get_field("version"),
        }
    }
}
