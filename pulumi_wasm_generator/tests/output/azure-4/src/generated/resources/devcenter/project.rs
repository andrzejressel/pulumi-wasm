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
pub mod project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Description of the project. Changing this forces a new Dev Center Project to be created.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Resource Id of an associated DevCenter. Changing this forces a new Dev Center Project to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Azure Region where the Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When specified, limits the maximum number of Dev Boxes a single user can create across all pools in the project.
        #[builder(into, default)]
        pub maximum_dev_boxes_per_user: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of this Dev Center Project. Changing this forces a new Dev Center Project to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Dev Center Project.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// Description of the project. Changing this forces a new Dev Center Project to be created.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource Id of an associated DevCenter. Changing this forces a new Dev Center Project to be created.
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// The URI of the Dev Center resource this project is associated with.
        pub dev_center_uri: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// When specified, limits the maximum number of Dev Boxes a single user can create across all pools in the project.
        pub maximum_dev_boxes_per_user: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of this Dev Center Project. Changing this forces a new Dev Center Project to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Dev Center Project should exist. Changing this forces a new Dev Center Project to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Project.
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
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let dev_center_id_binding = args.dev_center_id.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let maximum_dev_boxes_per_user_binding = args
            .maximum_dev_boxes_per_user
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maximumDevBoxesPerUser".into(),
                    value: &maximum_dev_boxes_per_user_binding,
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
        ProjectResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dev_center_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("devCenterId"),
            ),
            dev_center_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("devCenterUri"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maximum_dev_boxes_per_user: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maximumDevBoxesPerUser"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
