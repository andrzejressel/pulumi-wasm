/// Manages a Spacecraft.
///
/// > **Note:** The `azure.orbital.Spacecraft` resource has been deprecated and will be removed in v5.0 of the AzureRM Provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleSpacecraft:
///     type: azure:orbital:Spacecraft
///     name: example
///     properties:
///       name: example-spacecraft
///       resourceGroupName: ${example.name}
///       location: westeurope
///       noradId: '12345'
///       links:
///         - bandwidthMhz: 30
///           centerFrequencyMhz: 2050
///           direction: Uplink
///           polarization: LHCP
///           name: examplename
///       twoLineElements:
///         - 1 23455U 94089A   97320.90946019  .00000140  00000-0  10191-3 0  2621
///         - 2 23455  99.0090 272.6745 0008546 223.1686 136.8816 14.11711747148495
///       titleLine: AQUA
///       tags:
///         aks-managed-cluster-name: 9a57225d-a405-4d40-aa46-f13d2342abef
/// ```
///
/// ## Import
///
/// Spacecraft can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:orbital/spacecraft:Spacecraft example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Orbital/spacecrafts/spacecraft1
/// ```
///
pub mod spacecraft {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpacecraftArgs {
        /// A `links` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub links: pulumi_wasm_rust::Output<
            Vec<super::super::types::orbital::SpacecraftLink>,
        >,
        /// The location where the Spacecraft exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Spacecraft. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// NORAD ID of the Spacecraft.
        #[builder(into)]
        pub norad_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Spacecraft exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Title of the two line elements (TLE).
        #[builder(into)]
        pub title_line: pulumi_wasm_rust::Output<String>,
        /// A list of the two line elements (TLE), the first string being the first of the TLE, the second string being the second line of the TLE. Changing this forces a new resource to be created.
        #[builder(into)]
        pub two_line_elements: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct SpacecraftResult {
        /// A `links` block as defined below. Changing this forces a new resource to be created.
        pub links: pulumi_wasm_rust::Output<
            Vec<super::super::types::orbital::SpacecraftLink>,
        >,
        /// The location where the Spacecraft exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Spacecraft. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// NORAD ID of the Spacecraft.
        pub norad_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Spacecraft exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Title of the two line elements (TLE).
        pub title_line: pulumi_wasm_rust::Output<String>,
        /// A list of the two line elements (TLE), the first string being the first of the TLE, the second string being the second line of the TLE. Changing this forces a new resource to be created.
        pub two_line_elements: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SpacecraftArgs) -> SpacecraftResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let links_binding = args.links.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let norad_id_binding = args.norad_id.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let title_line_binding = args.title_line.get_inner();
        let two_line_elements_binding = args.two_line_elements.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:orbital/spacecraft:Spacecraft".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "links".into(),
                    value: &links_binding,
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
                    name: "noradId".into(),
                    value: &norad_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "titleLine".into(),
                    value: &title_line_binding,
                },
                register_interface::ObjectField {
                    name: "twoLineElements".into(),
                    value: &two_line_elements_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "links".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "noradId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "titleLine".into(),
                },
                register_interface::ResultField {
                    name: "twoLineElements".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpacecraftResult {
            links: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("links").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            norad_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noradId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            title_line: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("titleLine").unwrap(),
            ),
            two_line_elements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("twoLineElements").unwrap(),
            ),
        }
    }
}
