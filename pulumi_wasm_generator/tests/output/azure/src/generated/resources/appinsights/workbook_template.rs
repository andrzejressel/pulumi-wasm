/// Manages an Application Insights Workbook Template.
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
///   exampleWorkbookTemplate:
///     type: azure:appinsights:WorkbookTemplate
///     name: example
///     properties:
///       name: example-aiwt
///       resourceGroupName: ${example.name}
///       location: West Europe
///       author: test author
///       priority: 1
///       galleries:
///         - category: workbook
///           name: test
///           order: 100
///           resourceType: microsoft.insights/components
///           type: tsg
///       templateData:
///         fn::toJSON:
///           version: Notebook/1.0
///           items:
///             - type: 1
///               content:
///                 json: |-
///                   ## New workbook
///                   ---
///
///                   Welcome to your new workbook.
///               name: text - 2
///           styleSettings: {}
///           $schema: https://github.com/Microsoft/Application-Insights-Workbooks/blob/master/schema/workbook.json
///       localized:
///         fn::toJSON:
///           ar:
///             - galleries:
///                 - name: test
///                   category: Failures
///                   type: tsg
///                   resourceType: microsoft.insights/components
///                   order: 100
///               templateData:
///                 version: Notebook/1.0
///                 items:
///                   - type: 1
///                     content:
///                       json: |-
///                         ## New workbook
///                         ---
///
///                         Welcome to your new workbook.
///                     name: text - 2
///                 styleSettings: {}
///                 $schema: https://github.com/Microsoft/Application-Insights-Workbooks/blob/master/schema/workbook.json
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Application Insights Workbook Template can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/workbookTemplate:WorkbookTemplate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Insights/workbookTemplates/resource1
/// ```
///
pub mod workbook_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkbookTemplateArgs {
        /// Information about the author of the workbook template.
        #[builder(into, default)]
        pub author: pulumi_wasm_rust::Output<Option<String>>,
        /// A `galleries` block as defined below.
        #[builder(into)]
        pub galleries: pulumi_wasm_rust::Output<
            Vec<super::super::types::appinsights::WorkbookTemplateGallery>,
        >,
        /// Key value pairs of localized gallery. Each key is the locale code of languages supported by the Azure portal.
        #[builder(into, default)]
        pub localized: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Application Insights Workbook Template. Changing this forces a new Application Insights Workbook Template to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Priority of the template. Determines which template to open when a workbook gallery is opened in viewer mode. Defaults to `0`.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Resource Group where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Insights Workbook Template.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Valid JSON object containing workbook template payload.
        #[builder(into)]
        pub template_data: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkbookTemplateResult {
        /// Information about the author of the workbook template.
        pub author: pulumi_wasm_rust::Output<Option<String>>,
        /// A `galleries` block as defined below.
        pub galleries: pulumi_wasm_rust::Output<
            Vec<super::super::types::appinsights::WorkbookTemplateGallery>,
        >,
        /// Key value pairs of localized gallery. Each key is the locale code of languages supported by the Azure portal.
        pub localized: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Application Insights Workbook Template. Changing this forces a new Application Insights Workbook Template to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Priority of the template. Determines which template to open when a workbook gallery is opened in viewer mode. Defaults to `0`.
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Resource Group where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Insights Workbook Template.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Valid JSON object containing workbook template payload.
        pub template_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkbookTemplateArgs) -> WorkbookTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let author_binding = args.author.get_inner();
        let galleries_binding = args.galleries.get_inner();
        let localized_binding = args.localized.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_data_binding = args.template_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appinsights/workbookTemplate:WorkbookTemplate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "author".into(),
                    value: &author_binding,
                },
                register_interface::ObjectField {
                    name: "galleries".into(),
                    value: &galleries_binding,
                },
                register_interface::ObjectField {
                    name: "localized".into(),
                    value: &localized_binding,
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
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "templateData".into(),
                    value: &template_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "author".into(),
                },
                register_interface::ResultField {
                    name: "galleries".into(),
                },
                register_interface::ResultField {
                    name: "localized".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "templateData".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkbookTemplateResult {
            author: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("author").unwrap(),
            ),
            galleries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleries").unwrap(),
            ),
            localized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localized").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            template_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateData").unwrap(),
            ),
        }
    }
}
