/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Dev Center Project.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:devcenter:DevCenter
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       identity:
///         type: example-value
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleProject:
///     type: azure:devcenter:Project
///     name: example
///     properties:
///       devCenterId: ${example.id}
///       location: ${exampleResourceGroup.location}
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// An existing Dev Center Project can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/project:Project example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DevCenter/projects/{projectName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Dev Center Project exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Dev Center Project exists. For example `example-resource-group`.
///
/// * Where `{projectName}` is the name of the Project. For example `projectValue`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Description of the project. Changing this forces a new Dev Center Project to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource Id of an associated DevCenter. Changing this forces a new Dev Center Project to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When specified, limits the maximum number of Dev Boxes a single user can create across all pools in the project.
        #[builder(into, default)]
        pub maximum_dev_boxes_per_user: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of this Dev Center Project. Changing this forces a new Dev Center Project to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Dev Center Project.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// Description of the project. Changing this forces a new Dev Center Project to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource Id of an associated DevCenter. Changing this forces a new Dev Center Project to be created.
        pub dev_center_id: pulumi_gestalt_rust::Output<String>,
        /// The URI of the Dev Center resource this project is associated with.
        pub dev_center_uri: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// When specified, limits the maximum number of Dev Boxes a single user can create across all pools in the project.
        pub maximum_dev_boxes_per_user: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of this Dev Center Project. Changing this forces a new Dev Center Project to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Project.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let dev_center_id_binding = args.dev_center_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let maximum_dev_boxes_per_user_binding = args
            .maximum_dev_boxes_per_user
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumDevBoxesPerUser".into(),
                    value: &maximum_dev_boxes_per_user_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            description: o.get_field("description"),
            dev_center_id: o.get_field("devCenterId"),
            dev_center_uri: o.get_field("devCenterUri"),
            location: o.get_field("location"),
            maximum_dev_boxes_per_user: o.get_field("maximumDevBoxesPerUser"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
