/// Manages a proximity placement group for virtual machines, virtual machine scale sets and availability sets.
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
///   examplePlacementGroup:
///     type: azure:proximity:PlacementGroup
///     name: example
///     properties:
///       name: exampleProximityPlacementGroup
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Proximity Placement Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:proximity/placementGroup:PlacementGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Compute/proximityPlacementGroups/example-ppg
/// ```
///
pub mod placement_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlacementGroupArgs {
        /// Specifies the supported sizes of Virtual Machines that can be created in the Proximity Placement Group.
        ///
        /// > **NOTE:** Removing `allowed_vm_sizes` after it is set forces a new resource to be created.
        #[builder(into, default)]
        pub allowed_vm_sizes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the proximity placement group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the availability set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the supported zone of the Proximity Placement Group. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `allowed_vm_sizes` must be set when `zone` is specified.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PlacementGroupResult {
        /// Specifies the supported sizes of Virtual Machines that can be created in the Proximity Placement Group.
        ///
        /// > **NOTE:** Removing `allowed_vm_sizes` after it is set forces a new resource to be created.
        pub allowed_vm_sizes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the proximity placement group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the availability set. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the supported zone of the Proximity Placement Group. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `allowed_vm_sizes` must be set when `zone` is specified.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PlacementGroupArgs) -> PlacementGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_vm_sizes_binding = args.allowed_vm_sizes.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:proximity/placementGroup:PlacementGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedVmSizes".into(),
                    value: &allowed_vm_sizes_binding,
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
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedVmSizes".into(),
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
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PlacementGroupResult {
            allowed_vm_sizes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedVmSizes").unwrap(),
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
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}