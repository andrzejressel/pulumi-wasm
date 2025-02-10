/// Allows associating hierarchical firewall policies with the target where they are applied. This allows creating policies and rules in a different location than they are applied.
///
/// For more information on applying hierarchical firewall policies see the [official documentation](https://cloud.google.com/vpc/docs/firewall-policies#managing_hierarchical_firewall_policy_resources)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = firewall_policy::create(
///         "default",
///         FirewallPolicyArgs::builder()
///             .description("Example Resource")
///             .parent("organizations/12345")
///             .short_name("my-policy")
///             .build_struct(),
///     );
///     let defaultFirewallPolicyAssociation = firewall_policy_association::create(
///         "defaultFirewallPolicyAssociation",
///         FirewallPolicyAssociationArgs::builder()
///             .attachment_target("${folder.name}")
///             .firewall_policy("${default.id}")
///             .name("my-association")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FirewallPolicyAssociation can be imported using any of these accepted formats:
///
/// * `locations/global/firewallPolicies/{{firewall_policy}}/associations/{{name}}`
///
/// * `{{firewall_policy}}/{{name}}`
///
/// When using the `pulumi import` command, FirewallPolicyAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicyAssociation:FirewallPolicyAssociation default locations/global/firewallPolicies/{{firewall_policy}}/associations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicyAssociation:FirewallPolicyAssociation default {{firewall_policy}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyAssociationArgs {
        /// The target that the firewall policy is attached to.
        #[builder(into)]
        pub attachment_target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The firewall policy ID of the association.
        #[builder(into)]
        pub firewall_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for an association.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyAssociationResult {
        /// The target that the firewall policy is attached to.
        pub attachment_target: pulumi_gestalt_rust::Output<String>,
        /// The firewall policy ID of the association.
        pub firewall_policy: pulumi_gestalt_rust::Output<String>,
        /// The name for an association.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
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
        args: FirewallPolicyAssociationArgs,
    ) -> FirewallPolicyAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attachment_target_binding = args.attachment_target.get_output(context);
        let firewall_policy_binding = args.firewall_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/firewallPolicyAssociation:FirewallPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachmentTarget".into(),
                    value: attachment_target_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicy".into(),
                    value: firewall_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallPolicyAssociationResult {
            attachment_target: o.get_field("attachmentTarget"),
            firewall_policy: o.get_field("firewallPolicy"),
            name: o.get_field("name"),
            short_name: o.get_field("shortName"),
        }
    }
}
