/// Provides a Route 53 Resolver DNSSEC config resource.
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
///     let exampleResolverDnsSecConfig = resolver_dns_sec_config::create(
///         "exampleResolverDnsSecConfig",
///         ResolverDnsSecConfigArgs::builder().resource_id("${example.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import  Route 53 Resolver DNSSEC configs using the Route 53 Resolver DNSSEC config ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverDnsSecConfig:ResolverDnsSecConfig example rdsc-be1866ecc1683e95
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_dns_sec_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverDnsSecConfigArgs {
        /// The ID of the virtual private cloud (VPC) that you're updating the DNSSEC validation status for.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverDnsSecConfigResult {
        /// The ARN for a configuration for DNSSEC validation.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The owner account ID of the virtual private cloud (VPC) for a configuration for DNSSEC validation.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the virtual private cloud (VPC) that you're updating the DNSSEC validation status for.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// The validation status for a DNSSEC configuration. The status can be one of the following: `ENABLING`, `ENABLED`, `DISABLING` and `DISABLED`.
        pub validation_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverDnsSecConfigArgs,
    ) -> ResolverDnsSecConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_id_binding = args.resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/resolverDnsSecConfig:ResolverDnsSecConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverDnsSecConfigResult {
            arn: o.get_field("arn"),
            owner_id: o.get_field("ownerId"),
            resource_id: o.get_field("resourceId"),
            validation_status: o.get_field("validationStatus"),
        }
    }
}
