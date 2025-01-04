/// Manage a Dedicated Host Group.
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
///             .name("example-rg-compute")
///             .build_struct(),
///     );
///     let exampleDedicatedHostGroup = dedicated_host_group::create(
///         "exampleDedicatedHostGroup",
///         DedicatedHostGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-dedicated-host-group")
///             .platform_fault_domain_count(1)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Dedicated Host Group can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/dedicatedHostGroup:DedicatedHostGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Compute/hostGroups/group1
/// ```
///
pub mod dedicated_host_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedHostGroupArgs {
        /// Would virtual machines or virtual machine scale sets be placed automatically on this Dedicated Host Group? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub automatic_placement_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure location where the Dedicated Host Group exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Dedicated Host Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of fault domains that the Dedicated Host Group spans. Changing this forces a new resource to be created.
        #[builder(into)]
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the resource group the Dedicated Host Group is located in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone in which this Dedicated Host Group should be located. Changing this forces a new Dedicated Host Group to be created.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DedicatedHostGroupResult {
        /// Would virtual machines or virtual machine scale sets be placed automatically on this Dedicated Host Group? Defaults to `false`. Changing this forces a new resource to be created.
        pub automatic_placement_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure location where the Dedicated Host Group exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Dedicated Host Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of fault domains that the Dedicated Host Group spans. Changing this forces a new resource to be created.
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the resource group the Dedicated Host Group is located in. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone in which this Dedicated Host Group should be located. Changing this forces a new Dedicated Host Group to be created.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DedicatedHostGroupArgs) -> DedicatedHostGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_placement_enabled_binding = args
            .automatic_placement_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/dedicatedHostGroup:DedicatedHostGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticPlacementEnabled".into(),
                    value: &automatic_placement_enabled_binding,
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
                    name: "platformFaultDomainCount".into(),
                    value: &platform_fault_domain_count_binding,
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
                    name: "automaticPlacementEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platformFaultDomainCount".into(),
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
        DedicatedHostGroupResult {
            automatic_placement_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticPlacementEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform_fault_domain_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformFaultDomainCount").unwrap(),
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
