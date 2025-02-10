#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcConnectionArgs {
        /// ARN of the VPC Connection.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of key-value pairs assigned to the VPC Connection.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcConnectionResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The authentication type for the client VPC Connection.
        pub authentication: pulumi_gestalt_rust::Output<String>,
        /// The list of subnets in the client VPC.
        pub client_subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The security groups attached to the ENIs for the broker nodes.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of key-value pairs assigned to the VPC Connection.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Amazon Resource Name (ARN) of the cluster.
        pub target_cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// The VPC ID of the remote client.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcConnectionArgs,
    ) -> GetVpcConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:msk/getVpcConnection:getVpcConnection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcConnectionResult {
            arn: o.get_field("arn"),
            authentication: o.get_field("authentication"),
            client_subnets: o.get_field("clientSubnets"),
            id: o.get_field("id"),
            security_groups: o.get_field("securityGroups"),
            tags: o.get_field("tags"),
            target_cluster_arn: o.get_field("targetClusterArn"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
