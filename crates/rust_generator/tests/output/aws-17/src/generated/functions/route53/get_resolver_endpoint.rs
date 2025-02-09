#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverEndpointArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [Route53resolver Filter value in the AWS API reference][1].
        ///
        /// In addition to all arguments above, the following attributes are exported:
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::route53::GetResolverEndpointFilter>>,
        >,
        /// ID of the Route53 Resolver Endpoint.
        #[builder(into, default)]
        pub resolver_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResolverEndpointResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub direction: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::route53::GetResolverEndpointFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resolver_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub resolver_endpoint_type: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResolverEndpointArgs,
    ) -> GetResolverEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding_1 = args.filters.get_output(context);
        let filters_binding = filters_binding_1.get_inner();
        let resolver_endpoint_id_binding_1 = args
            .resolver_endpoint_id
            .get_output(context);
        let resolver_endpoint_id_binding = resolver_endpoint_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverEndpoint:getResolverEndpoint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "resolverEndpointId".into(),
                    value: &resolver_endpoint_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverEndpointResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            direction: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("direction"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddresses"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            resolver_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resolverEndpointId"),
            ),
            resolver_endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resolverEndpointType"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
