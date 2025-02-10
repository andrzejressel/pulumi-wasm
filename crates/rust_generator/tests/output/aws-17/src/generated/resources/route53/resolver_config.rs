/// Provides a Route 53 Resolver config resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc::create(
///         "example",
///         VpcArgs::builder()
///             .cidr_block("10.0.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let exampleResolverConfig = resolver_config::create(
///         "exampleResolverConfig",
///         ResolverConfigArgs::builder()
///             .autodefined_reverse_flag("DISABLE")
///             .resource_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Resolver configs using the Route 53 Resolver config ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverConfig:ResolverConfig example rslvr-rc-715aa20c73a23da7
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverConfigArgs {
        /// Indicates whether or not the Resolver will create autodefined rules for reverse DNS lookups. Valid values: `ENABLE`, `DISABLE`.
        #[builder(into)]
        pub autodefined_reverse_flag: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC that the configuration is for.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverConfigResult {
        /// Indicates whether or not the Resolver will create autodefined rules for reverse DNS lookups. Valid values: `ENABLE`, `DISABLE`.
        pub autodefined_reverse_flag: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the owner of the VPC that this resolver configuration applies to.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC that the configuration is for.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverConfigArgs,
    ) -> ResolverConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autodefined_reverse_flag_binding = args
            .autodefined_reverse_flag
            .get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/resolverConfig:ResolverConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autodefinedReverseFlag".into(),
                    value: autodefined_reverse_flag_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: resource_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverConfigResult {
            autodefined_reverse_flag: o.get_field("autodefinedReverseFlag"),
            owner_id: o.get_field("ownerId"),
            resource_id: o.get_field("resourceId"),
        }
    }
}
