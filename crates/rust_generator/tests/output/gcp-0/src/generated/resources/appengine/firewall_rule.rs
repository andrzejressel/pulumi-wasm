/// A single firewall rule that is evaluated against incoming traffic
/// and provides an action to take on matched requests.
///
///
/// To get more information about FirewallRule, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.firewall.ingressRules)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/appengine/docs/standard/python/creating-firewalls#creating_firewall_rules)
///
/// ## Example Usage
///
/// ### App Engine Firewall Rule Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = application::create(
///         "app",
///         ApplicationArgs::builder()
///             .location_id("us-central")
///             .project("${myProject.projectId}")
///             .build_struct(),
///     );
///     let myProject = project::create(
///         "myProject",
///         ProjectArgs::builder()
///             .billing_account("000000-0000000-0000000-000000")
///             .deletion_policy("DELETE")
///             .name("tf-test-project")
///             .org_id("123456789")
///             .project_id("ae-project")
///             .build_struct(),
///     );
///     let rule = firewall_rule::create(
///         "rule",
///         FirewallRuleArgs::builder()
///             .action("ALLOW")
///             .priority(1000)
///             .project("${app.project}")
///             .source_range("*")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FirewallRule can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/firewall/ingressRules/{{priority}}`
///
/// * `{{project}}/{{priority}}`
///
/// * `{{priority}}`
///
/// When using the `pulumi import` command, FirewallRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/firewallRule:FirewallRule default apps/{{project}}/firewall/ingressRules/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/firewallRule:FirewallRule default {{project}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/firewallRule:FirewallRule default {{priority}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// The action to take if this rule matches.
        /// Possible values are: `UNSPECIFIED_ACTION`, `ALLOW`, `DENY`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional string description of this rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A positive integer that defines the order of rule evaluation.
        /// Rules with the lowest priority are evaluated first.
        /// A default rule at priority Int32.MaxValue matches all IPv4 and
        /// IPv6 traffic when no previous rule matches. Only the action of
        /// this rule can be modified by the user.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP address or range, defined using CIDR notation, of requests that this rule applies to.
        #[builder(into)]
        pub source_range: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// The action to take if this rule matches.
        /// Possible values are: `UNSPECIFIED_ACTION`, `ALLOW`, `DENY`.
        ///
        ///
        /// - - -
        pub action: pulumi_gestalt_rust::Output<String>,
        /// An optional string description of this rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A positive integer that defines the order of rule evaluation.
        /// Rules with the lowest priority are evaluated first.
        /// A default rule at priority Int32.MaxValue matches all IPv4 and
        /// IPv6 traffic when no previous rule matches. Only the action of
        /// this rule can be modified by the user.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// IP address or range, defined using CIDR notation, of requests that this rule applies to.
        pub source_range: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallRuleArgs,
    ) -> FirewallRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let project_binding = args.project.get_output(context);
        let source_range_binding = args.source_range.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/firewallRule:FirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: &action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRange".into(),
                    value: &source_range_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallRuleResult {
            action: o.get_field("action"),
            description: o.get_field("description"),
            priority: o.get_field("priority"),
            project: o.get_field("project"),
            source_range: o.get_field("sourceRange"),
        }
    }
}
