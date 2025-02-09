/// Manages a Firewall Policy Rule Collection Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFirewallPolicy:
///     type: azure:network:FirewallPolicy
///     name: example
///     properties:
///       name: example-fwpolicy
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleFirewallPolicyRuleCollectionGroup:
///     type: azure:network:FirewallPolicyRuleCollectionGroup
///     name: example
///     properties:
///       name: example-fwpolicy-rcg
///       firewallPolicyId: ${exampleFirewallPolicy.id}
///       priority: 500
///       applicationRuleCollections:
///         - name: app_rule_collection1
///           priority: 500
///           action: Deny
///           rules:
///             - name: app_rule_collection1_rule1
///               protocols:
///                 - type: Http
///                   port: 80
///                 - type: Https
///                   port: 443
///               sourceAddresses:
///                 - 10.0.0.1
///               destinationFqdns:
///                 - '*.microsoft.com'
///       networkRuleCollections:
///         - name: network_rule_collection1
///           priority: 400
///           action: Deny
///           rules:
///             - name: network_rule_collection1_rule1
///               protocols:
///                 - TCP
///                 - UDP
///               sourceAddresses:
///                 - 10.0.0.1
///               destinationAddresses:
///                 - 192.168.1.1
///                 - 192.168.1.2
///               destinationPorts:
///                 - '80'
///                 - 1000-2000
///       natRuleCollections:
///         - name: nat_rule_collection1
///           priority: 300
///           action: Dnat
///           rules:
///             - name: nat_rule_collection1_rule1
///               protocols:
///                 - TCP
///                 - UDP
///               sourceAddresses:
///                 - 10.0.0.1
///                 - 10.0.0.2
///               destinationAddress: 192.168.1.1
///               destinationPorts: '80'
///               translatedAddress: 192.168.0.1
///               translatedPort: '8080'
/// ```
///
/// ## Import
///
/// Firewall Policy Rule Collection Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/firewallPolicyRuleCollectionGroup:FirewallPolicyRuleCollectionGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/firewallPolicies/policy1/ruleCollectionGroups/gruop1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy_rule_collection_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleCollectionGroupArgs {
        /// One or more `application_rule_collection` blocks as defined below.
        #[builder(into, default)]
        pub application_rule_collections: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollection,
                >,
            >,
        >,
        /// The ID of the Firewall Policy where the Firewall Policy Rule Collection Group should exist. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        #[builder(into)]
        pub firewall_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `nat_rule_collection` blocks as defined below.
        #[builder(into, default)]
        pub nat_rule_collections: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNatRuleCollection,
                >,
            >,
        >,
        /// One or more `network_rule_collection` blocks as defined below.
        #[builder(into, default)]
        pub network_rule_collections: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNetworkRuleCollection,
                >,
            >,
        >,
        /// The priority of the Firewall Policy Rule Collection Group. The range is 100-65000.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleCollectionGroupResult {
        /// One or more `application_rule_collection` blocks as defined below.
        pub application_rule_collections: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollection,
                >,
            >,
        >,
        /// The ID of the Firewall Policy where the Firewall Policy Rule Collection Group should exist. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        pub firewall_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `nat_rule_collection` blocks as defined below.
        pub nat_rule_collections: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNatRuleCollection,
                >,
            >,
        >,
        /// One or more `network_rule_collection` blocks as defined below.
        pub network_rule_collections: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNetworkRuleCollection,
                >,
            >,
        >,
        /// The priority of the Firewall Policy Rule Collection Group. The range is 100-65000.
        pub priority: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FirewallPolicyRuleCollectionGroupArgs,
    ) -> FirewallPolicyRuleCollectionGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_rule_collections_binding_1 = args
            .application_rule_collections
            .get_output(context);
        let application_rule_collections_binding = application_rule_collections_binding_1
            .get_inner();
        let firewall_policy_id_binding_1 = args.firewall_policy_id.get_output(context);
        let firewall_policy_id_binding = firewall_policy_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let nat_rule_collections_binding_1 = args
            .nat_rule_collections
            .get_output(context);
        let nat_rule_collections_binding = nat_rule_collections_binding_1.get_inner();
        let network_rule_collections_binding_1 = args
            .network_rule_collections
            .get_output(context);
        let network_rule_collections_binding = network_rule_collections_binding_1
            .get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/firewallPolicyRuleCollectionGroup:FirewallPolicyRuleCollectionGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationRuleCollections".into(),
                    value: &application_rule_collections_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicyId".into(),
                    value: &firewall_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "natRuleCollections".into(),
                    value: &nat_rule_collections_binding,
                },
                register_interface::ObjectField {
                    name: "networkRuleCollections".into(),
                    value: &network_rule_collections_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallPolicyRuleCollectionGroupResult {
            application_rule_collections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationRuleCollections"),
            ),
            firewall_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallPolicyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nat_rule_collections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("natRuleCollections"),
            ),
            network_rule_collections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkRuleCollections"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
        }
    }
}
