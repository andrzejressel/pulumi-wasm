/// Manages a maintenance assignment to Dedicated Host.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAssignmentDedicatedHost = assignment_dedicated_host::create(
///         "exampleAssignmentDedicatedHost",
///         AssignmentDedicatedHostArgs::builder()
///             .dedicated_host_id("${exampleDedicatedHost.id}")
///             .location("${example.location}")
///             .maintenance_configuration_id("${exampleConfiguration.id}")
///             .build_struct(),
///     );
///     let exampleConfiguration = configuration::create(
///         "exampleConfiguration",
///         ConfigurationArgs::builder()
///             .location("${example.location}")
///             .name("example-mc")
///             .resource_group_name("${example.name}")
///             .scope("Host")
///             .build_struct(),
///     );
///     let exampleDedicatedHost = dedicated_host::create(
///         "exampleDedicatedHost",
///         DedicatedHostArgs::builder()
///             .dedicated_host_group_id("${exampleDedicatedHostGroup.id}")
///             .location("${example.location}")
///             .name("example-host")
///             .platform_fault_domain(1)
///             .sku_name("DSv3-Type3")
///             .build_struct(),
///     );
///     let exampleDedicatedHostGroup = dedicated_host_group::create(
///         "exampleDedicatedHostGroup",
///         DedicatedHostGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-host-group")
///             .platform_fault_domain_count(2)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Maintenance Assignment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maintenance/assignmentDedicatedHost:AssignmentDedicatedHost example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Compute/hostGroups/group1/hosts/host1/providers/Microsoft.Maintenance/configurationAssignments/assign1
/// ```
///
pub mod assignment_dedicated_host {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentDedicatedHostArgs {
        /// Specifies the Dedicated Host ID to which the Maintenance Configuration will be assigned. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dedicated_host_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Maintenance Configuration Resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub maintenance_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AssignmentDedicatedHostResult {
        /// Specifies the Dedicated Host ID to which the Maintenance Configuration will be assigned. Changing this forces a new resource to be created.
        pub dedicated_host_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Maintenance Configuration Resource. Changing this forces a new resource to be created.
        pub maintenance_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AssignmentDedicatedHostArgs,
    ) -> AssignmentDedicatedHostResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dedicated_host_id_binding = args.dedicated_host_id.get_inner();
        let location_binding = args.location.get_inner();
        let maintenance_configuration_id_binding = args
            .maintenance_configuration_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:maintenance/assignmentDedicatedHost:AssignmentDedicatedHost"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dedicatedHostId".into(),
                    value: &dedicated_host_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfigurationId".into(),
                    value: &maintenance_configuration_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dedicatedHostId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceConfigurationId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssignmentDedicatedHostResult {
            dedicated_host_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedHostId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceConfigurationId").unwrap(),
            ),
        }
    }
}