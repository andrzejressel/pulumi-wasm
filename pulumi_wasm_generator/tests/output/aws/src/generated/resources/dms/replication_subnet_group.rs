/// Provides a DMS (Data Migration Service) replication subnet group resource. DMS replication subnet groups can be created, updated, deleted, and imported.
///
/// > **Note:** AWS requires a special IAM role called `dms-vpc-role` when using this resource. See the example below to create it as part of your configuration.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   # Create a new replication subnet group
///   example:
///     type: aws:dms:ReplicationSubnetGroup
///     properties:
///       replicationSubnetGroupDescription: Example replication subnet group
///       replicationSubnetGroupId: example-dms-replication-subnet-group-tf
///       subnetIds:
///         - subnet-12345678
///         - subnet-12345679
///       tags:
///         Name: example
/// ```
///
/// ### Creating special IAM role
///
/// If your account does not already include the `dms-vpc-role` IAM role, you will need to create it to allow DMS to manage subnets in the VPC.
///
/// ```yaml
/// resources:
///   dms-vpc-role:
///     type: aws:iam:Role
///     properties:
///       name: dms-vpc-role
///       description: Allows DMS to manage VPC
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Effect: Allow
///               Principal:
///                 Service: dms.amazonaws.com
///               Action: sts:AssumeRole
///   example:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       role: ${["dms-vpc-role"].name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AmazonDMSVPCManagementRole
///   exampleReplicationSubnetGroup:
///     type: aws:dms:ReplicationSubnetGroup
///     name: example
///     properties:
///       replicationSubnetGroupDescription: Example
///       replicationSubnetGroupId: example-id
///       subnetIds:
///         - subnet-12345678
///         - subnet-12345679
///       tags:
///         Name: example-id
///     options:
///       dependsOn:
///         - ${example}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import replication subnet groups using the `replication_subnet_group_id`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/replicationSubnetGroup:ReplicationSubnetGroup test test-dms-replication-subnet-group-tf
/// ```
pub mod replication_subnet_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationSubnetGroupArgs {
        /// Description for the subnet group.
        #[builder(into)]
        pub replication_subnet_group_description: pulumi_wasm_rust::Output<String>,
        /// Name for the replication subnet group. This value is stored as a lowercase string. It must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens and cannot be `default`.
        #[builder(into)]
        pub replication_subnet_group_id: pulumi_wasm_rust::Output<String>,
        /// List of at least 2 EC2 subnet IDs for the subnet group. The subnets must cover at least 2 availability zones.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationSubnetGroupResult {
        pub replication_subnet_group_arn: pulumi_wasm_rust::Output<String>,
        /// Description for the subnet group.
        pub replication_subnet_group_description: pulumi_wasm_rust::Output<String>,
        /// Name for the replication subnet group. This value is stored as a lowercase string. It must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens and cannot be `default`.
        pub replication_subnet_group_id: pulumi_wasm_rust::Output<String>,
        /// List of at least 2 EC2 subnet IDs for the subnet group. The subnets must cover at least 2 availability zones.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC the subnet group is in.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ReplicationSubnetGroupArgs,
    ) -> ReplicationSubnetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let replication_subnet_group_description_binding = args
            .replication_subnet_group_description
            .get_inner();
        let replication_subnet_group_id_binding = args
            .replication_subnet_group_id
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/replicationSubnetGroup:ReplicationSubnetGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationSubnetGroupDescription".into(),
                    value: &replication_subnet_group_description_binding,
                },
                register_interface::ObjectField {
                    name: "replicationSubnetGroupId".into(),
                    value: &replication_subnet_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationSubnetGroupResult {
            replication_subnet_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupArn").unwrap(),
            ),
            replication_subnet_group_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupDescription").unwrap(),
            ),
            replication_subnet_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationSubnetGroupId").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
