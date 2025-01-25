/// Manages an Availability Set for Virtual Machines.
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
///   exampleAvailabilitySet:
///     type: azure:compute:AvailabilitySet
///     name: example
///     properties:
///       name: example-aset
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Availability Sets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/availabilitySet:AvailabilitySet group1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/availabilitySets/webAvailSet
/// ```
///
pub mod availability_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AvailabilitySetArgs {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the availability set is managed or not. Possible values are `true` (to specify aligned) or `false` (to specify classic). Default is `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the availability set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the number of fault domains that are used. Defaults to `3`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The number of Fault Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        #[builder(into, default)]
        pub platform_fault_domain_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the number of update domains that are used. Defaults to `5`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The number of Update Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        #[builder(into, default)]
        pub platform_update_domain_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub proximity_placement_group_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the resource group in which to create the availability set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AvailabilitySetResult {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the availability set is managed or not. Possible values are `true` (to specify aligned) or `false` (to specify classic). Default is `true`. Changing this forces a new resource to be created.
        pub managed: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the availability set. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of fault domains that are used. Defaults to `3`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The number of Fault Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the number of update domains that are used. Defaults to `5`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The number of Update Domains varies depending on which Azure Region you're using. More information about update and fault domains and how they work can be found [here](https://learn.microsoft.com/en-us/azure/virtual-machines/availability-set-overview).
        pub platform_update_domain_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Proximity Placement Group to which this Virtual Machine should be assigned. Changing this forces a new resource to be created.
        pub proximity_placement_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the availability set. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AvailabilitySetArgs,
    ) -> AvailabilitySetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let managed_binding = args.managed.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_output(context)
            .get_inner();
        let platform_update_domain_count_binding = args
            .platform_update_domain_count
            .get_output(context)
            .get_inner();
        let proximity_placement_group_id_binding = args
            .proximity_placement_group_id
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/availabilitySet:AvailabilitySet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managed".into(),
                    value: &managed_binding,
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
                    name: "platformUpdateDomainCount".into(),
                    value: &platform_update_domain_count_binding,
                },
                register_interface::ObjectField {
                    name: "proximityPlacementGroupId".into(),
                    value: &proximity_placement_group_id_binding,
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
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managed".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platformFaultDomainCount".into(),
                },
                register_interface::ResultField {
                    name: "platformUpdateDomainCount".into(),
                },
                register_interface::ResultField {
                    name: "proximityPlacementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AvailabilitySetResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managed").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform_fault_domain_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformFaultDomainCount").unwrap(),
            ),
            platform_update_domain_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformUpdateDomainCount").unwrap(),
            ),
            proximity_placement_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proximityPlacementGroupId").unwrap(),
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
