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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DevBoxDefinitionArgs,
    ) -> DevBoxDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dev_center_id_binding = args.dev_center_id.get_output(context).get_inner();
        let image_reference_id_binding = args
            .image_reference_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/devBoxDefinition:DevBoxDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding,
                },
                register_interface::ObjectField {
                    name: "imageReferenceId".into(),
                    value: &image_reference_id_binding,
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
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DevBoxDefinitionResult {
            dev_center_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("devCenterId"),
            ),
            image_reference_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageReferenceId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
