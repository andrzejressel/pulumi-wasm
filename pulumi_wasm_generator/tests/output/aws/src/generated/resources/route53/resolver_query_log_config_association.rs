/// Provides a Route 53 Resolver query logging configuration association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resolver_query_log_config_association::create(
///         "example",
///         ResolverQueryLogConfigAssociationArgs::builder()
///             .resolver_query_log_config_id(
///                 "${exampleAwsRoute53ResolverQueryLogConfig.id}",
///             )
///             .resource_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import  Route 53 Resolver query logging configuration associations using the Route 53 Resolver query logging configuration association ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverQueryLogConfigAssociation:ResolverQueryLogConfigAssociation example rqlca-b320624fef3c4d70
/// ```
pub mod resolver_query_log_config_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverQueryLogConfigAssociationArgs {
        /// The ID of the Route 53 Resolver query logging configuration that you want to associate a VPC with.
        #[builder(into)]
        pub resolver_query_log_config_id: pulumi_wasm_rust::Output<String>,
        /// The ID of a VPC that you want this query logging configuration to log queries for.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverQueryLogConfigAssociationResult {
        /// The ID of the Route 53 Resolver query logging configuration that you want to associate a VPC with.
        pub resolver_query_log_config_id: pulumi_wasm_rust::Output<String>,
        /// The ID of a VPC that you want this query logging configuration to log queries for.
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResolverQueryLogConfigAssociationArgs,
    ) -> ResolverQueryLogConfigAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resolver_query_log_config_id_binding = args
            .resolver_query_log_config_id
            .get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverQueryLogConfigAssociation:ResolverQueryLogConfigAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resolverQueryLogConfigId".into(),
                    value: &resolver_query_log_config_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resolverQueryLogConfigId".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResolverQueryLogConfigAssociationResult {
            resolver_query_log_config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resolverQueryLogConfigId").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}