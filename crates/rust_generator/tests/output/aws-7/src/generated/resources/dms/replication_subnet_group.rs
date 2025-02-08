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
#[allow(clippy::doc_lazy_continuation)]
pub mod replication_subnet_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationSubnetGroupArgs {
        /// Description for the subnet group.
        #[builder(into)]
        pub replication_subnet_group_description: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Name for the replication subnet group. This value is stored as a lowercase string. It must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens and cannot be `default`.
        #[builder(into)]
        pub replication_subnet_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of at least 2 EC2 subnet IDs for the subnet group. The subnets must cover at least 2 availability zones.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationSubnetGroupResult {
        pub replication_subnet_group_arn: pulumi_gestalt_rust::Output<String>,
        /// Description for the subnet group.
        pub replication_subnet_group_description: pulumi_gestalt_rust::Output<String>,
        /// Name for the replication subnet group. This value is stored as a lowercase string. It must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens and cannot be `default`.
        pub replication_subnet_group_id: pulumi_gestalt_rust::Output<String>,
        /// List of at least 2 EC2 subnet IDs for the subnet group. The subnets must cover at least 2 availability zones.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC the subnet group is in.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ReplicationSubnetGroupArgs,
    ) -> ReplicationSubnetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let replication_subnet_group_description_binding = args
            .replication_subnet_group_description
            .get_output(context)
            .get_inner();
        let replication_subnet_group_id_binding = args
            .replication_subnet_group_id
            .get_output(context)
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/replicationSubnetGroup:ReplicationSubnetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReplicationSubnetGroupResult {
            replication_subnet_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationSubnetGroupArn"),
            ),
            replication_subnet_group_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationSubnetGroupDescription"),
            ),
            replication_subnet_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationSubnetGroupId"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
