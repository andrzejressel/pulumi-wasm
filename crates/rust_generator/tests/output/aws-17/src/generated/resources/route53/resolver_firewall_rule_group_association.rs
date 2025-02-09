/// Provides a Route 53 Resolver DNS Firewall rule group association resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resolver_firewall_rule_group::create(
///         "example",
///         ResolverFirewallRuleGroupArgs::builder().name("example").build_struct(),
///     );
///     let exampleResolverFirewallRuleGroupAssociation = resolver_firewall_rule_group_association::create(
///         "exampleResolverFirewallRuleGroupAssociation",
///         ResolverFirewallRuleGroupAssociationArgs::builder()
///             .firewall_rule_group_id("${example.id}")
///             .name("example")
///             .priority(100)
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Resolver DNS Firewall rule group associations using the Route 53 Resolver DNS Firewall rule group association ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverFirewallRuleGroupAssociation:ResolverFirewallRuleGroupAssociation example rslvr-frgassoc-0123456789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_firewall_rule_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverFirewallRuleGroupAssociationArgs {
        /// The unique identifier of the firewall rule group.
        #[builder(into)]
        pub firewall_rule_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub mutation_protection: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name that lets you identify the rule group association, to manage and use it.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The setting that determines the processing order of the rule group among the rule groups that you associate with the specified VPC. DNS Firewall filters VPC traffic starting from the rule group with the lowest numeric priority setting.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique identifier of the VPC that you want to associate with the rule group.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverFirewallRuleGroupAssociationResult {
        /// The ARN (Amazon Resource Name) of the firewall rule group association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the firewall rule group.
        pub firewall_rule_group_id: pulumi_gestalt_rust::Output<String>,
        /// If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. Valid values: `ENABLED`, `DISABLED`.
        pub mutation_protection: pulumi_gestalt_rust::Output<String>,
        /// A name that lets you identify the rule group association, to manage and use it.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The setting that determines the processing order of the rule group among the rule groups that you associate with the specified VPC. DNS Firewall filters VPC traffic starting from the rule group with the lowest numeric priority setting.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique identifier of the VPC that you want to associate with the rule group.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResolverFirewallRuleGroupAssociationArgs,
    ) -> ResolverFirewallRuleGroupAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let firewall_rule_group_id_binding_1 = args
            .firewall_rule_group_id
            .get_output(context);
        let firewall_rule_group_id_binding = firewall_rule_group_id_binding_1
            .get_inner();
        let mutation_protection_binding_1 = args.mutation_protection.get_output(context);
        let mutation_protection_binding = mutation_protection_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let vpc_id_binding_1 = args.vpc_id.get_output(context);
        let vpc_id_binding = vpc_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverFirewallRuleGroupAssociation:ResolverFirewallRuleGroupAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallRuleGroupId".into(),
                    value: &firewall_rule_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "mutationProtection".into(),
                    value: &mutation_protection_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResolverFirewallRuleGroupAssociationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            firewall_rule_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallRuleGroupId"),
            ),
            mutation_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mutationProtection"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
