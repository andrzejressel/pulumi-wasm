#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_subnet_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationSubnetGroupArgs {
        /// Name for the replication subnet group. This value is stored as a lowercase string. It must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens and cannot be `default`.
        #[builder(into)]
        pub replication_subnet_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationSubnetGroupResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub replication_subnet_group_arn: pulumi_gestalt_rust::Output<String>,
        /// Description for the subnet group.
        pub replication_subnet_group_description: pulumi_gestalt_rust::Output<String>,
        pub replication_subnet_group_id: pulumi_gestalt_rust::Output<String>,
        pub subnet_group_status: pulumi_gestalt_rust::Output<String>,
        /// List of at least 2 EC2 subnet IDs for the subnet group. The subnets must cover at least 2 availability zones.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the VPC the subnet group is in.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReplicationSubnetGroupArgs,
    ) -> GetReplicationSubnetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let replication_subnet_group_id_binding = args
            .replication_subnet_group_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dms/getReplicationSubnetGroup:getReplicationSubnetGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationSubnetGroupId".into(),
                    value: replication_subnet_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReplicationSubnetGroupResult {
            id: o.get_field("id"),
            replication_subnet_group_arn: o.get_field("replicationSubnetGroupArn"),
            replication_subnet_group_description: o
                .get_field("replicationSubnetGroupDescription"),
            replication_subnet_group_id: o.get_field("replicationSubnetGroupId"),
            subnet_group_status: o.get_field("subnetGroupStatus"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
