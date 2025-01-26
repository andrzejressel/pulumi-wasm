/// Manages an Azure Workbook.
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
///   exampleWorkbook:
///     type: azure:appinsights:Workbook
///     name: example
///     properties:
///       name: 85b3e8bb-fc93-40be-83f2-98f6bec18ba0
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       displayName: workbook1
///       dataJson:
///         fn::toJSON:
///           version: Notebook/1.0
///           items:
///             - type: 1
///               content:
///                 json: Test2022
///               name: text - 0
///           isLocked: false
///           fallbackResourceIds:
///             - Azure Monitor
///       tags:
///         ENV: Test
/// ```
///
/// ## Import
///
/// Workbooks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/workbook:Workbook example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Insights/workbooks/resource1
/// ```
///
pub mod workbook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkbookArgs {
        /// Workbook category, as defined by the user at creation time. There may be additional category types beyond the following: `workbook`, `sentinel`. Defaults to `workbook`.
        #[builder(into, default)]
        pub category: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration of this particular workbook. Configuration data is a string containing valid JSON.
        #[builder(into)]
        pub data_json: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the description of the workbook.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the user-defined name (display name) of the workbook.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// An `identity` block as defined below. Changing this forces a new Workbook to be created.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appinsights::WorkbookIdentity>,
        >,
        /// Specifies the Azure Region where the Workbook should exist. Changing this forces a new Workbook to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Workbook as a UUID/GUID. It should not contain any uppercase letters. Changing this forces a new Workbook to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Workbook should exist. Changing this forces a new Workbook to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Resource ID for a source resource. It should not contain any uppercase letters. Defaults to `azure monitor`.
        #[builder(into, default)]
        pub source_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Manager ID of the Storage Container when bring your own storage is used. Changing this forces a new Workbook to be created.
        ///
        /// > **Note:** This is the Resource Manager ID of the Storage Container, rather than the regular ID - and can be accessed on the `azure.storage.Container` Data Source/Resource as `resource_manager_id`.
        #[builder(into, default)]
        pub storage_container_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Workbook.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkbookResult {
        /// Workbook category, as defined by the user at creation time. There may be additional category types beyond the following: `workbook`, `sentinel`. Defaults to `workbook`.
        pub category: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration of this particular workbook. Configuration data is a string containing valid JSON.
        pub data_json: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the workbook.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the user-defined name (display name) of the workbook.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Workbook to be created.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::appinsights::WorkbookIdentity>,
        >,
        /// Specifies the Azure Region where the Workbook should exist. Changing this forces a new Workbook to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Workbook as a UUID/GUID. It should not contain any uppercase letters. Changing this forces a new Workbook to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Workbook should exist. Changing this forces a new Workbook to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Resource ID for a source resource. It should not contain any uppercase letters. Defaults to `azure monitor`.
        pub source_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Resource Manager ID of the Storage Container when bring your own storage is used. Changing this forces a new Workbook to be created.
        ///
        /// > **Note:** This is the Resource Manager ID of the Storage Container, rather than the regular ID - and can be accessed on the `azure.storage.Container` Data Source/Resource as `resource_manager_id`.
        pub storage_container_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Workbook.
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
        args: WorkbookArgs,
    ) -> WorkbookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let category_binding = args.category.get_output(context).get_inner();
        let data_json_binding = args.data_json.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let source_id_binding = args.source_id.get_output(context).get_inner();
        let storage_container_id_binding = args
            .storage_container_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appinsights/workbook:Workbook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "category".into(),
                    value: &category_binding,
                },
                register_interface::ObjectField {
                    name: "dataJson".into(),
                    value: &data_json_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
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
                    name: "sourceId".into(),
                    value: &source_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerId".into(),
                    value: &storage_container_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "category".into(),
                },
                register_interface::ResultField {
                    name: "dataJson".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceId".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkbookResult {
            category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("category").unwrap(),
            ),
            data_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataJson").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceId").unwrap(),
            ),
            storage_container_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
