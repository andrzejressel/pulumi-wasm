pub mod get_node_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNodeGroupArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Name of the node group.
        #[builder(into)]
        pub node_group_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNodeGroupResult {
        /// Type of Amazon Machine Image (AMI) associated with the EKS Node Group.
        pub ami_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the EKS Node Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Type of capacity associated with the EKS Node Group. Valid values: `ON_DEMAND`, `SPOT`.
        pub capacity_type: pulumi_wasm_rust::Output<String>,
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Disk size in GiB for worker nodes.
        pub disk_size: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of instance types associated with the EKS Node Group.
        pub instance_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of Kubernetes labels. Only labels that are applied with the EKS API are managed by this argument. Other Kubernetes labels applied to the EKS Node Group will not be managed.
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Nested attribute containing information about the launch template used to create the EKS Node Group.
        pub launch_templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupLaunchTemplate>,
        >,
        pub node_group_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM Role that provides permissions for the EKS Node Group.
        pub node_role_arn: pulumi_wasm_rust::Output<String>,
        /// AMI version of the EKS Node Group.
        pub release_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block with remote access settings.
        pub remote_accesses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupRemoteAccess>,
        >,
        /// List of objects containing information about underlying resources.
        pub resources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupResource>,
        >,
        /// Configuration block with scaling settings.
        pub scaling_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupScalingConfig>,
        >,
        /// Status of the EKS Node Group.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Identifiers of EC2 Subnets to associate with the EKS Node Group.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// List of objects containing information about taints applied to the nodes in the EKS Node Group.
        pub taints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetNodeGroupTaint>,
        >,
        /// Kubernetes version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNodeGroupArgs) -> GetNodeGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_inner();
        let node_group_name_binding = args.node_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:eks/getNodeGroup:getNodeGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeGroupName".into(),
                    value: &node_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "amiType".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "capacityType".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "diskSize".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceTypes".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "launchTemplates".into(),
                },
                register_interface::ResultField {
                    name: "nodeGroupName".into(),
                },
                register_interface::ResultField {
                    name: "nodeRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "releaseVersion".into(),
                },
                register_interface::ResultField {
                    name: "remoteAccesses".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "scalingConfigs".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "taints".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNodeGroupResult {
            ami_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amiType").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            capacity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityType").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            disk_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSize").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTypes").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            launch_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchTemplates").unwrap(),
            ),
            node_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeGroupName").unwrap(),
            ),
            node_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeRoleArn").unwrap(),
            ),
            release_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseVersion").unwrap(),
            ),
            remote_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteAccesses").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            scaling_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingConfigs").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            taints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taints").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
