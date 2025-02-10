#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_serverless_vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessVpcEndpointArgs {
        /// The unique identifier of the endpoint.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessVpcEndpointResult {
        /// The date the endpoint was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the endpoint.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The IDs of the security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The IDs of the subnets from which you access OpenSearch Serverless.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC from which you access OpenSearch Serverless.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServerlessVpcEndpointArgs,
    ) -> GetServerlessVpcEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:opensearch/getServerlessVpcEndpoint:getServerlessVpcEndpoint"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: vpc_endpoint_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServerlessVpcEndpointResult {
            created_date: o.get_field("createdDate"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
