/// Manages a Dev Center Environment Type.
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
///   exampleDevCenter:
///     type: azure:devcenter:DevCenter
///     name: example
///     properties:
///       name: example-dc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleEnvironmentType:
///     type: azure:devcenter:EnvironmentType
///     name: example
///     properties:
///       name: example-dcet
///       devCenterId: ${exampleDevCenter.id}
///       tags:
///         Env: Test
/// ```
///
/// ## Import
///
/// An existing Dev Center Environment Type can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/environmentType:EnvironmentType example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/devCenters/dc1/environmentTypes/et1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentTypeArgs {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of this Dev Center Environment Type. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Dev Center Environment Type.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentTypeResult {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        pub dev_center_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center Environment Type. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Environment Type.
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
        args: EnvironmentTypeArgs,
    ) -> EnvironmentTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dev_center_id_binding = args.dev_center_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/environmentType:EnvironmentType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentTypeResult {
            dev_center_id: o.get_field("devCenterId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
