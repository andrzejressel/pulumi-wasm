/// Manages a Storage Mover.
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
///   exampleMover:
///     type: azure:storage:Mover
///     name: example
///     properties:
///       name: example-ssm
///       resourceGroupName: ${example.name}
///       location: West Europe
///       description: Example Storage Mover Description
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Storage Mover can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/mover:Mover example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mover {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverArgs {
        /// A description for the Storage Mover.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Storage Mover should exist. Changing this forces a new Storage Mover to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover. Changing this forces a new Storage Mover to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Storage Mover should exist. Changing this forces a new Storage Mover to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Storage Mover.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MoverResult {
        /// A description for the Storage Mover.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Storage Mover should exist. Changing this forces a new Storage Mover to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Storage Mover. Changing this forces a new Storage Mover to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Storage Mover should exist. Changing this forces a new Storage Mover to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Storage Mover.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MoverArgs,
    ) -> MoverResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/mover:Mover".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MoverResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
