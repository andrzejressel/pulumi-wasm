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
pub mod dev_box_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevBoxDefinitionArgs {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the image for the Dev Center Dev Box Definition.
        #[builder(into)]
        pub image_reference_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Dev Center Dev Box Definition should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of this Dev Center Dev Box Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the SKU for the Dev Center Dev Box Definition.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Dev Box Definition.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DevBoxDefinitionResult {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the image for the Dev Center Dev Box Definition.
        pub image_reference_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Dev Center Dev Box Definition should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Dev Center Dev Box Definition. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the SKU for the Dev Center Dev Box Definition.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Dev Box Definition.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DevBoxDefinitionArgs) -> DevBoxDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dev_center_id_binding = args.dev_center_id.get_inner();
        let image_reference_id_binding = args.image_reference_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/devBoxDefinition:DevBoxDefinition".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "devCenterId".into(),
                },
                register_interface::ResultField {
                    name: "imageReferenceId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DevBoxDefinitionResult {
            dev_center_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devCenterId").unwrap(),
            ),
            image_reference_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageReferenceId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}