/// A Response Policy Rule is a selector that applies its behavior to queries that match the selector.
/// Selectors are DNS names, which may be wildcards or exact matches.
/// Each DNS query subject to a Response Policy matches at most one ResponsePolicyRule,
/// as identified by the dns_name field with the longest matching suffix.
///
///
///
/// ## Example Usage
///
/// ### Dns Response Policy Rule Basic
///
///
/// ```yaml
/// resources:
///   network-1:
///     type: gcp:compute:Network
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
///   network-2:
///     type: gcp:compute:Network
///     properties:
///       name: network-2
///       autoCreateSubnetworks: false
///   response-policy:
///     type: gcp:dns:ResponsePolicy
///     properties:
///       responsePolicyName: example-response-policy
///       networks:
///         - networkUrl: ${["network-1"].id}
///         - networkUrl: ${["network-2"].id}
///   example-response-policy-rule:
///     type: gcp:dns:ResponsePolicyRule
///     properties:
///       responsePolicy: ${["response-policy"].responsePolicyName}
///       ruleName: example-rule
///       dnsName: dns.example.com.
///       localData:
///         localDatas:
///           - name: dns.example.com.
///             type: A
///             ttl: 300
///             rrdatas:
///               - 192.0.2.91
/// ```
///
/// ## Import
///
/// ResponsePolicyRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/responsePolicies/{{response_policy}}/rules/{{rule_name}}`
///
/// * `{{project}}/{{response_policy}}/{{rule_name}}`
///
/// * `{{response_policy}}/{{rule_name}}`
///
/// When using the `pulumi import` command, ResponsePolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicyRule:ResponsePolicyRule default projects/{{project}}/responsePolicies/{{response_policy}}/rules/{{rule_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicyRule:ResponsePolicyRule default {{project}}/{{response_policy}}/{{rule_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dns/responsePolicyRule:ResponsePolicyRule default {{response_policy}}/{{rule_name}}
/// ```
///
pub mod response_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponsePolicyRuleArgs {
        /// Answer this query with a behavior rather than DNS data. Acceptable values are 'behaviorUnspecified', and 'bypassResponsePolicy'
        #[builder(into, default)]
        pub behavior: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule.
        #[builder(into)]
        pub dns_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name;
        /// in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed.
        /// Structure is documented below.
        #[builder(into, default)]
        pub local_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dns::ResponsePolicyRuleLocalData>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifies the response policy addressed by this request.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub response_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An identifier for this rule. Must be unique with the ResponsePolicy.
        #[builder(into)]
        pub rule_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResponsePolicyRuleResult {
        /// Answer this query with a behavior rather than DNS data. Acceptable values are 'behaviorUnspecified', and 'bypassResponsePolicy'
        pub behavior: pulumi_gestalt_rust::Output<Option<String>>,
        /// The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name;
        /// in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed.
        /// Structure is documented below.
        pub local_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::dns::ResponsePolicyRuleLocalData>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Identifies the response policy addressed by this request.
        ///
        ///
        /// - - -
        pub response_policy: pulumi_gestalt_rust::Output<String>,
        /// An identifier for this rule. Must be unique with the ResponsePolicy.
        pub rule_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResponsePolicyRuleArgs,
    ) -> ResponsePolicyRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let behavior_binding = args.behavior.get_output(context).get_inner();
        let dns_name_binding = args.dns_name.get_output(context).get_inner();
        let local_data_binding = args.local_data.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let response_policy_binding = args
            .response_policy
            .get_output(context)
            .get_inner();
        let rule_name_binding = args.rule_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dns/responsePolicyRule:ResponsePolicyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "behavior".into(),
                    value: &behavior_binding,
                },
                register_interface::ObjectField {
                    name: "dnsName".into(),
                    value: &dns_name_binding,
                },
                register_interface::ObjectField {
                    name: "localData".into(),
                    value: &local_data_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "responsePolicy".into(),
                    value: &response_policy_binding,
                },
                register_interface::ObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResponsePolicyRuleResult {
            behavior: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("behavior"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            local_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localData"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            response_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responsePolicy"),
            ),
            rule_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleName"),
            ),
        }
    }
}
