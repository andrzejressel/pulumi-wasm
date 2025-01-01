/// Manages a network security group that contains a list of network security rules.  Network security groups enable inbound or outbound traffic to be enabled or denied.
///
/// > **NOTE on Network Security Groups and Network Security Rules:** This provider currently
/// provides both a standalone Network Security Rule resource, and allows for Network Security Rules to be defined in-line within the Network Security Group resource.
/// At this time you cannot use a Network Security Group with in-line Network Security Rules in conjunction with any Network Security Rule resources. Doing so will cause a conflict of rule settings and will overwrite rules.
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
///   exampleNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: example
///     properties:
///       name: acceptanceTestSecurityGroup1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       securityRules:
///         - name: test123
///           priority: 100
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '*'
///           sourceAddressPrefix: '*'
///           destinationAddressPrefix: '*'
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Network Security Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkSecurityGroup:NetworkSecurityGroup group1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkSecurityGroups/mySecurityGroup
/// ```
///
pub mod network_security_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSecurityGroupArgs {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the security rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the network security group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of objects representing security rules, as defined below.
        ///
        /// > **NOTE** Since `security_rule` can be configured both inline and via the separate `azure.network.NetworkSecurityRule` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub security_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::NetworkSecurityGroupSecurityRule>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSecurityGroupResult {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the security rule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the network security group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of objects representing security rules, as defined below.
        ///
        /// > **NOTE** Since `security_rule` can be configured both inline and via the separate `azure.network.NetworkSecurityRule` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub security_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkSecurityGroupSecurityRule>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkSecurityGroupArgs,
    ) -> NetworkSecurityGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let security_rules_binding = args.security_rules.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkSecurityGroup:NetworkSecurityGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "securityRules".into(),
                    value: &security_rules_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "securityRules".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkSecurityGroupResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            security_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityRules").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
