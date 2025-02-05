/// Manages a Cloud Exadata Infrastructure.
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
///     let exampleExadataInfrastructure = exadata_infrastructure::create(
///         "exampleExadataInfrastructure",
///         ExadataInfrastructureArgs::builder()
///             .compute_count(2)
///             .display_name("example-exadata-infra")
///             .location("${example.location}")
///             .name("example-exadata-infra")
///             .resource_group_name("${example.name}")
///             .shape("Exadata.X9M")
///             .storage_count(3)
///             .zones(vec!["1",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cloud Exadata Infrastructures can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:oracle/exadataInfrastructure:ExadataInfrastructure example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup/providers/Oracle.Database/cloudExadataInfrastructures/cloudExadataInfrastructures1
/// ```
///
pub mod exadata_infrastructure {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExadataInfrastructureArgs {
        /// The number of compute servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub compute_count: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The email address used by Oracle to send notifications regarding databases and infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub customer_contacts: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The user-friendly name for the Cloud Exadata Infrastructure resource. The name does not need to be unique. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Azure Region where the Cloud Exadata Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `maintenance_window` blocks as defined below. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub maintenance_windows: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::oracle::ExadataInfrastructureMaintenanceWindow>,
            >,
        >,
        /// The name which should be used for this Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the ODB@A Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The shape of the ODB@A infrastructure resource. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub shape: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of storage servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub storage_count: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A mapping of tags which should be assigned to the Cloud Exadata Infrastructure.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Cloud Exadata Infrastructure zones. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub zones: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ExadataInfrastructureResult {
        /// The number of compute servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub compute_count: pulumi_wasm_rust::Output<i32>,
        /// The email address used by Oracle to send notifications regarding databases and infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub customer_contacts: pulumi_wasm_rust::Output<Vec<String>>,
        /// The user-friendly name for the Cloud Exadata Infrastructure resource. The name does not need to be unique. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Cloud Exadata Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// One or more `maintenance_window` blocks as defined below. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub maintenance_windows: pulumi_wasm_rust::Output<
            Vec<super::super::types::oracle::ExadataInfrastructureMaintenanceWindow>,
        >,
        /// The name which should be used for this Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the ODB@A Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The shape of the ODB@A infrastructure resource. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub shape: pulumi_wasm_rust::Output<String>,
        /// The number of storage servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub storage_count: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags which should be assigned to the Cloud Exadata Infrastructure.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Cloud Exadata Infrastructure zones. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExadataInfrastructureArgs,
    ) -> ExadataInfrastructureResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compute_count_binding = args.compute_count.get_output(context).get_inner();
        let customer_contacts_binding = args
            .customer_contacts
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let maintenance_windows_binding = args
            .maintenance_windows
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let shape_binding = args.shape.get_output(context).get_inner();
        let storage_count_binding = args.storage_count.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zones_binding = args.zones.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:oracle/exadataInfrastructure:ExadataInfrastructure".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computeCount".into(),
                    value: &compute_count_binding,
                },
                register_interface::ObjectField {
                    name: "customerContacts".into(),
                    value: &customer_contacts_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindows".into(),
                    value: &maintenance_windows_binding,
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
                    name: "shape".into(),
                    value: &shape_binding,
                },
                register_interface::ObjectField {
                    name: "storageCount".into(),
                    value: &storage_count_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExadataInfrastructureResult {
            compute_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeCount"),
            ),
            customer_contacts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerContacts"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maintenance_windows: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceWindows"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            shape: pulumi_wasm_rust::__private::into_domain(o.extract_field("shape")),
            storage_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageCount"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            zones: pulumi_wasm_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
