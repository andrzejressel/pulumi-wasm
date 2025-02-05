pub mod get_serverless_vpc_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessVpcEndpointArgs {
        /// The unique identifier of the endpoint.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessVpcEndpointResult {
        /// The date the endpoint was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the endpoint.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The IDs of the security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The IDs of the subnets from which you access OpenSearch Serverless.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC from which you access OpenSearch Serverless.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServerlessVpcEndpointArgs,
    ) -> GetServerlessVpcEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let vpc_endpoint_id_binding = args
            .vpc_endpoint_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:opensearch/getServerlessVpcEndpoint:getServerlessVpcEndpoint"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerlessVpcEndpointResult {
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcEndpointId"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
