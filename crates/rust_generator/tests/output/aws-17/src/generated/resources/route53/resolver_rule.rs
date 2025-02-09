/// Provides a Route53 Resolver rule.
///
/// ## Example Usage
///
/// ### System rule
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sys = resolver_rule::create(
///         "sys",
///         ResolverRuleArgs::builder()
///             .domain_name("subdomain.example.com")
///             .rule_type("SYSTEM")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Forward rule
///
/// ```yaml
/// resources:
///   fwd:
///     type: aws:route53:ResolverRule
///     properties:
///       domainName: example.com
///       name: example
///       ruleType: FORWARD
///       resolverEndpointId: ${foo.id}
///       targetIps:
///         - ip: 123.45.67.89
///       tags:
///         Environment: Prod
/// ```
///
/// ### IPv6 Forward rule
///
/// ```yaml
/// resources:
///   fwd:
///     type: aws:route53:ResolverRule
///     properties:
///       domainName: example.com
///       name: example
///       ruleType: FORWARD
///       resolverEndpointId: ${foo.id}
///       targetIps:
///         - ipv6: 2600:1f18:1686:2000:4e60:6e3e:258:da36
///       tags:
///         Environment: Prod
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Resolver rules using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverRule:ResolverRule sys rslvr-rr-0123456789abcdef0
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverRuleArgs {
        /// DNS queries for this domain name are forwarded to the IP addresses that are specified using `target_ip`.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Friendly name that lets you easily find a rule in the Resolver dashboard in the Route 53 console.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the outbound resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify using `target_ip`.
        /// This argument should only be specified for `FORWARD` type rules.
        #[builder(into, default)]
        pub resolver_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rule type. Valid values are `FORWARD`, `SYSTEM` and `RECURSIVE`.
        #[builder(into)]
        pub rule_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block(s) indicating the IPs that you want Resolver to forward DNS queries to (documented below).
        /// This argument should only be specified for `FORWARD` type rules.
        #[builder(into, default)]
        pub target_ips: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::ResolverRuleTargetIp>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResolverRuleResult {
        /// ARN (Amazon Resource Name) for the resolver rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// DNS queries for this domain name are forwarded to the IP addresses that are specified using `target_ip`.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Friendly name that lets you easily find a rule in the Resolver dashboard in the Route 53 console.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// When a rule is shared with another AWS account, the account ID of the account that the rule is shared with.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the outbound resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify using `target_ip`.
        /// This argument should only be specified for `FORWARD` type rules.
        pub resolver_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Rule type. Valid values are `FORWARD`, `SYSTEM` and `RECURSIVE`.
        pub rule_type: pulumi_gestalt_rust::Output<String>,
        /// Whether the rules is shared and, if so, whether the current account is sharing the rule with another account, or another account is sharing the rule with the current account.
        /// Values are `NOT_SHARED`, `SHARED_BY_ME` or `SHARED_WITH_ME`
        pub share_status: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block(s) indicating the IPs that you want Resolver to forward DNS queries to (documented below).
        /// This argument should only be specified for `FORWARD` type rules.
        pub target_ips: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::route53::ResolverRuleTargetIp>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverRuleArgs,
    ) -> ResolverRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resolver_endpoint_id_binding = args.resolver_endpoint_id.get_output(context);
        let rule_type_binding = args.rule_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_ips_binding = args.target_ips.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/resolverRule:ResolverRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resolverEndpointId".into(),
                    value: resolver_endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleType".into(),
                    value: rule_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetIps".into(),
                    value: target_ips_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverRuleResult {
            arn: o.get_field("arn"),
            domain_name: o.get_field("domainName"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            resolver_endpoint_id: o.get_field("resolverEndpointId"),
            rule_type: o.get_field("ruleType"),
            share_status: o.get_field("shareStatus"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_ips: o.get_field("targetIps"),
        }
    }
}
