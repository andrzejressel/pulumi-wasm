/// Manages a Mobile Network Data Network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: East Us
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkDataNetwork:
///     type: azure:mobile:NetworkDataNetwork
///     name: example
///     properties:
///       name: example-mndn
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       description: example description
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Data Network can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkDataNetwork:NetworkDataNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/mobileNetworks/mobileNetwork1/dataNetworks/dataNetwork1
/// ```
///
pub mod network_data_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkDataNetworkArgs {
        /// A description of this Mobile Network Data Network.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Mobile Network Data Network should exist. Changing this forces a new Mobile Network Data Network to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Mobile Network. Changing this forces a new Mobile Network Data Network to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Data Network. Changing this forces a new Mobile Network Data Network to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Data Network.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkDataNetworkResult {
        /// A description of this Mobile Network Data Network.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Mobile Network Data Network should exist. Changing this forces a new Mobile Network Data Network to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Mobile Network. Changing this forces a new Mobile Network Data Network to be created.
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Data Network. Changing this forces a new Mobile Network Data Network to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Data Network.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkDataNetworkArgs) -> NetworkDataNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let mobile_network_id_binding = args.mobile_network_id.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkDataNetwork:NetworkDataNetwork".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        NetworkDataNetworkResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}