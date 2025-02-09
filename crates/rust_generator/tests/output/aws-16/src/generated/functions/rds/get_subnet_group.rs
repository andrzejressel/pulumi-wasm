#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subnet_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetGroupArgs {
        /// Name of the RDS database subnet group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetGroupResult {
        /// ARN for the DB subnet group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Provides the description of the DB subnet group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Provides the status of the DB subnet group.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Contains a list of subnet identifiers.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The network type of the DB subnet group.
        pub supported_network_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Provides the VPC ID of the DB subnet group.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubnetGroupArgs,
    ) -> GetSubnetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getSubnetGroup:getSubnetGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubnetGroupResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            subnet_ids: o.get_field("subnetIds"),
            supported_network_types: o.get_field("supportedNetworkTypes"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
