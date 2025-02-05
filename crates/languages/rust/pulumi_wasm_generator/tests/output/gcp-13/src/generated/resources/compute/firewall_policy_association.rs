/// Allows associating hierarchical firewall policies with the target where they are applied. This allows creating policies and rules in a different location than they are applied.
///
/// For more information on applying hierarchical firewall policies see the [official documentation](https://cloud.google.com/vpc/docs/firewall-policies#managing_hierarchical_firewall_policy_resources)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod firewall_policy_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyAssociationArgs {
        /// The target that the firewall policy is attached to.
        #[builder(into)]
        pub attachment_target: pulumi_wasm_rust::InputOrOutput<String>,
        /// The firewall policy ID of the association.
        #[builder(into)]
        pub firewall_policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name for an association.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyAssociationResult {
        /// The target that the firewall policy is attached to.
        pub attachment_target: pulumi_wasm_rust::Output<String>,
        /// The firewall policy ID of the association.
        pub firewall_policy: pulumi_wasm_rust::Output<String>,
        /// The name for an association.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The short name of the firewall policy of the association.
        pub short_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallPolicyAssociationArgs,
    ) -> FirewallPolicyAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachment_target_binding = args
            .attachment_target
            .get_output(context)
            .get_inner();
        let firewall_policy_binding = args
            .firewall_policy
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/firewallPolicyAssociation:FirewallPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachmentTarget".into(),
                    value: &attachment_target_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicy".into(),
                    value: &firewall_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallPolicyAssociationResult {
            attachment_target: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("attachmentTarget"),
            ),
            firewall_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallPolicy"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            short_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
        }
    }
}
