/// Provides a Route53 Resolver rule association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resolver_rule_association::create(
///         "example",
///         ResolverRuleAssociationArgs::builder()
///             .resolver_rule_id("${sys.id}")
///             .vpc_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Resolver rule associations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverRuleAssociation:ResolverRuleAssociation example rslvr-rrassoc-97242eaf88example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_rule_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverRuleAssociationArgs {
        /// A name for the association that you're creating between a resolver rule and a VPC.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the resolver rule that you want to associate with the VPC.
        #[builder(into)]
        pub resolver_rule_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC that you want to associate the resolver rule with.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverRuleAssociationResult {
        /// A name for the association that you're creating between a resolver rule and a VPC.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the resolver rule that you want to associate with the VPC.
        pub resolver_rule_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC that you want to associate the resolver rule with.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResolverRuleAssociationArgs,
    ) -> ResolverRuleAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resolver_rule_id_binding_1 = args.resolver_rule_id.get_output(context);
        let resolver_rule_id_binding = resolver_rule_id_binding_1.get_inner();
        let vpc_id_binding_1 = args.vpc_id.get_output(context);
        let vpc_id_binding = vpc_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverRuleAssociation:ResolverRuleAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resolverRuleId".into(),
                    value: &resolver_rule_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResolverRuleAssociationResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resolver_rule_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resolverRuleId"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
