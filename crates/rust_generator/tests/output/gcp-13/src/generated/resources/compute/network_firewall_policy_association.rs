/// The Compute NetworkFirewallPolicyAssociation resource
///
///
/// To get more information about NetworkFirewallPolicyAssociation, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/networkFirewallPolicies/addAssociation)
///
/// ## Example Usage
///
/// ### Network Firewall Policy Association
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network_firewall_policy_association::create(
///         "default",
///         NetworkFirewallPolicyAssociationArgs::builder()
///             .attachment_target("${network.id}")
///             .firewall_policy("${policy.id}")
///             .name("my-association")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let policy = network_firewall_policy::create(
///         "policy",
///         NetworkFirewallPolicyArgs::builder()
///             .description("Sample global network firewall policy")
///             .name("my-policy")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetworkFirewallPolicyAssociation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/firewallPolicies/{{firewall_policy}}/associations/{{name}}`
///
/// * `{{project}}/{{firewall_policy}}/{{name}}`
///
/// * `{{firewall_policy}}/{{name}}`
///
/// When using the `pulumi import` command, NetworkFirewallPolicyAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/networkFirewallPolicyAssociation:NetworkFirewallPolicyAssociation default projects/{{project}}/global/firewallPolicies/{{firewall_policy}}/associations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkFirewallPolicyAssociation:NetworkFirewallPolicyAssociation default {{project}}/{{firewall_policy}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/networkFirewallPolicyAssociation:NetworkFirewallPolicyAssociation default {{firewall_policy}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_firewall_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkFirewallPolicyAssociationArgs {
        /// The target that the firewall policy is attached to.
        #[builder(into)]
        pub attachment_target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The firewall policy of the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub firewall_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for an association.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkFirewallPolicyAssociationResult {
        /// The target that the firewall policy is attached to.
        pub attachment_target: pulumi_gestalt_rust::Output<String>,
        /// The firewall policy of the resource.
        ///
        ///
        /// - - -
        pub firewall_policy: pulumi_gestalt_rust::Output<String>,
        /// The name for an association.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The short name of the firewall policy of the association.
        pub short_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkFirewallPolicyAssociationArgs,
    ) -> NetworkFirewallPolicyAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attachment_target_binding = args.attachment_target.get_output(context);
        let firewall_policy_binding = args.firewall_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/networkFirewallPolicyAssociation:NetworkFirewallPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachmentTarget".into(),
                    value: &attachment_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicy".into(),
                    value: &firewall_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkFirewallPolicyAssociationResult {
            attachment_target: o.get_field("attachmentTarget"),
            firewall_policy: o.get_field("firewallPolicy"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            short_name: o.get_field("shortName"),
        }
    }
}
