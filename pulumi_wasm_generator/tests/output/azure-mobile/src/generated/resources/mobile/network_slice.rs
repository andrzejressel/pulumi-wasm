/// Manages a Mobile Network Slice.
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
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkSlice:
///     type: azure:mobile:NetworkSlice
///     name: example
///     properties:
///       name: example-mns
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       description: an example slice
///       singleNetworkSliceSelectionAssistanceInformation:
///         sliceServiceType: 1
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Slice can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkSlice:NetworkSlice example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/mobileNetworks/mobileNetwork1/slices/slice1
/// ```
///
pub mod network_slice {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSliceArgs {
        /// A description for this Mobile Network Slice.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Mobile Network Slice should exist. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of Mobile Network which the Mobile Network Slice belongs to. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Slice. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `single_network_slice_selection_assistance_information` block as defined below. Single-network slice selection assistance information (S-NSSAI). Unique at the scope of a mobile network.
        #[builder(into)]
        pub single_network_slice_selection_assistance_information: pulumi_wasm_rust::Output<
            super::super::types::mobile::NetworkSliceSingleNetworkSliceSelectionAssistanceInformation,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Slice.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSliceResult {
        /// A description for this Mobile Network Slice.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Mobile Network Slice should exist. Changing this forces a new Mobile Network Slice to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of Mobile Network which the Mobile Network Slice belongs to. Changing this forces a new Mobile Network Slice to be created.
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Slice. Changing this forces a new Mobile Network Slice to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `single_network_slice_selection_assistance_information` block as defined below. Single-network slice selection assistance information (S-NSSAI). Unique at the scope of a mobile network.
        pub single_network_slice_selection_assistance_information: pulumi_wasm_rust::Output<
            super::super::types::mobile::NetworkSliceSingleNetworkSliceSelectionAssistanceInformation,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Slice.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkSliceArgs) -> NetworkSliceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let mobile_network_id_binding = args.mobile_network_id.get_inner();
        let name_binding = args.name.get_inner();
        let single_network_slice_selection_assistance_information_binding = args
            .single_network_slice_selection_assistance_information
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkSlice:NetworkSlice".into(),
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
                    name: "singleNetworkSliceSelectionAssistanceInformation".into(),
                    value: &single_network_slice_selection_assistance_information_binding,
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
                    name: "singleNetworkSliceSelectionAssistanceInformation".into(),
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
        NetworkSliceResult {
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
            single_network_slice_selection_assistance_information: pulumi_wasm_rust::__private::into_domain(
                hashmap
                    .remove("singleNetworkSliceSelectionAssistanceInformation")
                    .unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
