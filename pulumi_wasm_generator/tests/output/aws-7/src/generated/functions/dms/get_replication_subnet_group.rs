pub mod get_replication_subnet_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationSubnetGroupArgs {
        /// Name for the replication subnet group. This value is stored as a lowercase string. It must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens and cannot be `default`.
        #[builder(into)]
        pub replication_subnet_group_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationSubnetGroupResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub replication_subnet_group_arn: pulumi_wasm_rust::Output<String>,
        /// Description for the subnet group.
        pub replication_subnet_group_description: pulumi_wasm_rust::Output<String>,
        pub replication_subnet_group_id: pulumi_wasm_rust::Output<String>,
        pub subnet_group_status: pulumi_wasm_rust::Output<String>,
        /// List of at least 2 EC2 subnet IDs for the subnet group. The subnets must cover at least 2 availability zones.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the VPC the subnet group is in.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetReplicationSubnetGroupArgs,
    ) -> GetReplicationSubnetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let replication_subnet_group_id_binding = args
            .replication_subnet_group_id
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dms/getReplicationSubnetGroup:getReplicationSubnetGroup".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationSubnetGroupId".into(),
                    value: &replication_subnet_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "replicationSubnetGroupArn".into(),
                },
                register_interface::ResultField {
                    name: "replicationSubnetGroupDescription".into(),
                },
                register_interface::ResultField {
                    name: "replicationSubnetGroupId".into(),
                },
                register_interface::ResultField {
                    name: "subnetGroupStatus".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReplicationSubnetGroupResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            replication_subnet_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupArn").unwrap(),
            ),
            replication_subnet_group_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupDescription").unwrap(),
            ),
            replication_subnet_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupId").unwrap(),
            ),
            subnet_group_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetGroupStatus").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
