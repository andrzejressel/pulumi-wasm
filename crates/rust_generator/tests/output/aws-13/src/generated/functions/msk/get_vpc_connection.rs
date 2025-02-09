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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpcConnectionArgs,
    ) -> GetVpcConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding_1 = args.arn.get_output(context);
        let arn_binding = arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:msk/getVpcConnection:getVpcConnection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcConnectionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authentication"),
            ),
            client_subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientSubnets"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            target_cluster_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetClusterArn"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
