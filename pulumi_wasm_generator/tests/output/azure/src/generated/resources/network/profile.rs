/// Manages a Network Profile.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("examplegroup")
///             .build_struct(),
///     );
///     let exampleProfile = profile::create(
///         "exampleProfile",
///         ProfileArgs::builder()
///             .container_network_interface(
///                 ProfileContainerNetworkInterface::builder()
///                     .ipConfigurations(
///                         vec![
///                             ProfileContainerNetworkInterfaceIpConfiguration::builder()
///                             .name("exampleipconfig").subnetId("${exampleSubnet.id}")
///                             .build_struct(),
///                         ],
///                     )
///                     .name("examplecnic")
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .name("examplenetprofile")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.1.0.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("delegation")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/virtualNetworks/subnets/action",])
///                     .name("Microsoft.ContainerInstance/containerGroups").build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .name("examplesubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.1.0.0/16",])
///             .location("${example.location}")
///             .name("examplevnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Network Profile can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/profile:Profile example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/networkProfiles/examplenetprofile
/// ```
///
pub mod profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// A `container_network_interface` block as documented below.
        #[builder(into)]
        pub container_network_interface: pulumi_wasm_rust::Output<
            super::super::types::network::ProfileContainerNetworkInterface,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Network Profile. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// A `container_network_interface` block as documented below.
        pub container_network_interface: pulumi_wasm_rust::Output<
            super::super::types::network::ProfileContainerNetworkInterface,
        >,
        /// A list of Container Network Interface IDs.
        pub container_network_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Network Profile. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProfileArgs) -> ProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_network_interface_binding = args
            .container_network_interface
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/profile:Profile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerNetworkInterface".into(),
                    value: &container_network_interface_binding,
                },
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerNetworkInterface".into(),
                },
                register_interface::ResultField {
                    name: "containerNetworkInterfaceIds".into(),
                },
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
        ProfileResult {
            container_network_interface: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerNetworkInterface").unwrap(),
            ),
            container_network_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerNetworkInterfaceIds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}