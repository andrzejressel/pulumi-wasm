/// Provides a Route 53 Resolver query logging configuration association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_query_log_config_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverQueryLogConfigAssociationArgs {
        /// The ID of the Route 53 Resolver query logging configuration that you want to associate a VPC with.
        #[builder(into)]
        pub resolver_query_log_config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of a VPC that you want this query logging configuration to log queries for.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverQueryLogConfigAssociationResult {
        /// The ID of the Route 53 Resolver query logging configuration that you want to associate a VPC with.
        pub resolver_query_log_config_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of a VPC that you want this query logging configuration to log queries for.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverQueryLogConfigAssociationArgs,
    ) -> ResolverQueryLogConfigAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resolver_query_log_config_id_binding = args
            .resolver_query_log_config_id
            .get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/resolverQueryLogConfigAssociation:ResolverQueryLogConfigAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resolverQueryLogConfigId".into(),
                    value: resolver_query_log_config_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: resource_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverQueryLogConfigAssociationResult {
            resolver_query_log_config_id: o.get_field("resolverQueryLogConfigId"),
            resource_id: o.get_field("resourceId"),
        }
    }
}
