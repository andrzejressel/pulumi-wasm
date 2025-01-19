pub mod get_resolver_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverEndpointArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [Route53resolver Filter value in the AWS API reference][1].
        ///
        /// In addition to all arguments above, the following attributes are exported:
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::route53::GetResolverEndpointFilter>>,
        >,
        /// ID of the Route53 Resolver Endpoint.
        #[builder(into, default)]
        pub resolver_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResolverEndpointResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub direction: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::route53::GetResolverEndpointFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub protocols: pulumi_wasm_rust::Output<Vec<String>>,
        pub resolver_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        pub resolver_endpoint_type: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetResolverEndpointArgs) -> GetResolverEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let resolver_endpoint_id_binding = args.resolver_endpoint_id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "direction".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddresses".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protocols".into(),
                },
                register_interface::ResultField {
                    name: "resolverEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "resolverEndpointType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        GetResolverEndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            direction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("direction").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddresses").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocols").unwrap(),
            ),
            resolver_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resolverEndpointId").unwrap(),
            ),
            resolver_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resolverEndpointType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
