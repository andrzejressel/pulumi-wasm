/// Manages a Dev Center Dev Box Definition.
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
///   exampleDevBoxDefinition:
///     type: azure:devcenter:DevBoxDefinition
///     name: example
///     properties:
///       name: example-dcet
///       location: ${example.location}
///       devCenterId: ${exampleDevCenter.id}
///       imageReferenceId: ${exampleDevCenter.id}/galleries/default/images/microsoftvisualstudio_visualstudioplustools_vs-2022-ent-general-win10-m365-gen2
///       skuName: general_i_8c32gb256ssd_v2
/// ```
///
/// ## Import
///
/// An existing Dev Center Dev Box Definition can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/devBoxDefinition:DevBoxDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/devCenters/dc1/devBoxDefinitions/et1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dev_box_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevBoxDefinitionArgs {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the image for the Dev Center Dev Box Definition.
        #[builder(into)]
        pub image_reference_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Dev Center Dev Box Definition should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Dev Center Dev Box Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the SKU for the Dev Center Dev Box Definition.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Dev Center Dev Box Definition.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DevBoxDefinitionResult {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        pub dev_center_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the image for the Dev Center Dev Box Definition.
        pub image_reference_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Dev Center Dev Box Definition should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center Dev Box Definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the SKU for the Dev Center Dev Box Definition.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Dev Box Definition.
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
        args: DevBoxDefinitionArgs,
    ) -> DevBoxDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dev_center_id_binding = args.dev_center_id.get_output(context);
        let image_reference_id_binding = args.image_reference_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/devBoxDefinition:DevBoxDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devCenterId".into(),
                    value: dev_center_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageReferenceId".into(),
                    value: image_reference_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DevBoxDefinitionResult {
            dev_center_id: o.get_field("devCenterId"),
            image_reference_id: o.get_field("imageReferenceId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
        }
    }
}
