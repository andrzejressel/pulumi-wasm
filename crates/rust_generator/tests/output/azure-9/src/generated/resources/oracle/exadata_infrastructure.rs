/// Manages a Cloud Exadata Infrastructure.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod exadata_infrastructure {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExadataInfrastructureArgs {
        /// The number of compute servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub compute_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The email address used by Oracle to send notifications regarding databases and infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub customer_contacts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The user-friendly name for the Cloud Exadata Infrastructure resource. The name does not need to be unique. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Cloud Exadata Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `maintenance_window` blocks as defined below. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub maintenance_windows: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::oracle::ExadataInfrastructureMaintenanceWindow>,
            >,
        >,
        /// The name which should be used for this Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the ODB@A Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The shape of the ODB@A infrastructure resource. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub shape: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of storage servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub storage_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A mapping of tags which should be assigned to the Cloud Exadata Infrastructure.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Cloud Exadata Infrastructure zones. Changing this forces a new Cloud Exadata Infrastructure to be created.
        #[builder(into)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ExadataInfrastructureResult {
        /// The number of compute servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub compute_count: pulumi_gestalt_rust::Output<i32>,
        /// The email address used by Oracle to send notifications regarding databases and infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub customer_contacts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The user-friendly name for the Cloud Exadata Infrastructure resource. The name does not need to be unique. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Cloud Exadata Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// One or more `maintenance_window` blocks as defined below. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub maintenance_windows: pulumi_gestalt_rust::Output<
            Vec<super::super::types::oracle::ExadataInfrastructureMaintenanceWindow>,
        >,
        /// The name which should be used for this Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the ODB@A Infrastructure should exist. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The shape of the ODB@A infrastructure resource. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub shape: pulumi_gestalt_rust::Output<String>,
        /// The number of storage servers for the Cloud Exadata Infrastructure. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub storage_count: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags which should be assigned to the Cloud Exadata Infrastructure.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Cloud Exadata Infrastructure zones. Changing this forces a new Cloud Exadata Infrastructure to be created.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExadataInfrastructureArgs,
    ) -> ExadataInfrastructureResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compute_count_binding = args.compute_count.get_output(context);
        let customer_contacts_binding = args.customer_contacts.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_windows_binding = args.maintenance_windows.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let shape_binding = args.shape.get_output(context);
        let storage_count_binding = args.storage_count.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:oracle/exadataInfrastructure:ExadataInfrastructure".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeCount".into(),
                    value: compute_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerContacts".into(),
                    value: customer_contacts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceWindows".into(),
                    value: maintenance_windows_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shape".into(),
                    value: shape_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageCount".into(),
                    value: storage_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExadataInfrastructureResult {
            compute_count: o.get_field("computeCount"),
            customer_contacts: o.get_field("customerContacts"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            maintenance_windows: o.get_field("maintenanceWindows"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            shape: o.get_field("shape"),
            storage_count: o.get_field("storageCount"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
