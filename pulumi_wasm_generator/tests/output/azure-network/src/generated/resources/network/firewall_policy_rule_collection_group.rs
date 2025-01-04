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
pub mod firewall_policy_rule_collection_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleCollectionGroupArgs {
        /// One or more `application_rule_collection` blocks as defined below.
        #[builder(into, default)]
        pub application_rule_collections: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollection,
                >,
            >,
        >,
        /// The ID of the Firewall Policy where the Firewall Policy Rule Collection Group should exist. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        #[builder(into)]
        pub firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `nat_rule_collection` blocks as defined below.
        #[builder(into, default)]
        pub nat_rule_collections: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNatRuleCollection,
                >,
            >,
        >,
        /// One or more `network_rule_collection` blocks as defined below.
        #[builder(into, default)]
        pub network_rule_collections: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNetworkRuleCollection,
                >,
            >,
        >,
        /// The priority of the Firewall Policy Rule Collection Group. The range is 100-65000.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyRuleCollectionGroupResult {
        /// One or more `application_rule_collection` blocks as defined below.
        pub application_rule_collections: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollection,
                >,
            >,
        >,
        /// The ID of the Firewall Policy where the Firewall Policy Rule Collection Group should exist. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        pub firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `nat_rule_collection` blocks as defined below.
        pub nat_rule_collections: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNatRuleCollection,
                >,
            >,
        >,
        /// One or more `network_rule_collection` blocks as defined below.
        pub network_rule_collections: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::FirewallPolicyRuleCollectionGroupNetworkRuleCollection,
                >,
            >,
        >,
        /// The priority of the Firewall Policy Rule Collection Group. The range is 100-65000.
        pub priority: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FirewallPolicyRuleCollectionGroupArgs,
    ) -> FirewallPolicyRuleCollectionGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_rule_collections_binding = args
            .application_rule_collections
            .get_inner();
        let firewall_policy_id_binding = args.firewall_policy_id.get_inner();
        let name_binding = args.name.get_inner();
        let nat_rule_collections_binding = args.nat_rule_collections.get_inner();
        let network_rule_collections_binding = args.network_rule_collections.get_inner();
        let priority_binding = args.priority.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/firewallPolicyRuleCollectionGroup:FirewallPolicyRuleCollectionGroup"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationRuleCollections".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "natRuleCollections".into(),
                },
                register_interface::ResultField {
                    name: "networkRuleCollections".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallPolicyRuleCollectionGroupResult {
            application_rule_collections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationRuleCollections").unwrap(),
            ),
            firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nat_rule_collections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natRuleCollections").unwrap(),
            ),
            network_rule_collections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRuleCollections").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
        }
    }
}
