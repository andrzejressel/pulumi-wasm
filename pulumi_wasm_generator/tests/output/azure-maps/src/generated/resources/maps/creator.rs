/// Manages an Azure Maps Creator.
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
///   exampleAccount:
///     type: azure:maps:Account
///     name: example
///     properties:
///       name: example-maps-account
///       resourceGroupName: ${example.name}
///       skuName: G2
///       tags:
///         environment: Test
///   exampleCreator:
///     type: azure:maps:Creator
///     name: example
///     properties:
///       name: example-maps-creator
///       mapsAccountId: ${exampleAccount.id}
///       location: ${example.location}
///       storageUnits: 1
///       tags:
///         environment: Test
/// ```
///
/// ## Import
///
/// An Azure Maps Creators can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maps/creator:Creator example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Maps/accounts/account1/creators/creator1
/// ```
///
pub mod creator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CreatorArgs {
        /// The Azure Region where the Azure Maps Creator should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Azure Maps Creator. Changing this forces a new resource to be created.
        #[builder(into)]
        pub maps_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Azure Maps Creator. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The storage units to be allocated. Integer values from 1 to 100, inclusive.
        #[builder(into)]
        pub storage_units: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags which should be assigned to the Azure Maps Creator.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CreatorResult {
        /// The Azure Region where the Azure Maps Creator should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure Maps Creator. Changing this forces a new resource to be created.
        pub maps_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Azure Maps Creator. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The storage units to be allocated. Integer values from 1 to 100, inclusive.
        pub storage_units: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags which should be assigned to the Azure Maps Creator.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CreatorArgs) -> CreatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let maps_account_id_binding = args.maps_account_id.get_inner();
        let name_binding = args.name.get_inner();
        let storage_units_binding = args.storage_units.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:maps/creator:Creator".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mapsAccountId".into(),
                    value: &maps_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageUnits".into(),
                    value: &storage_units_binding,
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
                    name: "mapsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageUnits".into(),
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
        CreatorResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maps_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapsAccountId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageUnits").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
