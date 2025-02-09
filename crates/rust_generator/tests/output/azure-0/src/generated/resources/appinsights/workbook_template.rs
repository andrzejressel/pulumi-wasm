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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workbook_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkbookTemplateArgs {
        /// Information about the author of the workbook template.
        #[builder(into, default)]
        pub author: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `galleries` block as defined below.
        #[builder(into)]
        pub galleries: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::appinsights::WorkbookTemplateGallery>,
        >,
        /// Key value pairs of localized gallery. Each key is the locale code of languages supported by the Azure portal.
        #[builder(into, default)]
        pub localized: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Application Insights Workbook Template. Changing this forces a new Application Insights Workbook Template to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Priority of the template. Determines which template to open when a workbook gallery is opened in viewer mode. Defaults to `0`.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Resource Group where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Application Insights Workbook Template.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Valid JSON object containing workbook template payload.
        #[builder(into)]
        pub template_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkbookTemplateResult {
        /// Information about the author of the workbook template.
        pub author: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `galleries` block as defined below.
        pub galleries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appinsights::WorkbookTemplateGallery>,
        >,
        /// Key value pairs of localized gallery. Each key is the locale code of languages supported by the Azure portal.
        pub localized: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Application Insights Workbook Template. Changing this forces a new Application Insights Workbook Template to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Priority of the template. Determines which template to open when a workbook gallery is opened in viewer mode. Defaults to `0`.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Resource Group where the Application Insights Workbook Template should exist. Changing this forces a new Application Insights Workbook Template to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Insights Workbook Template.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Valid JSON object containing workbook template payload.
        pub template_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkbookTemplateArgs,
    ) -> WorkbookTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let author_binding_1 = args.author.get_output(context);
        let author_binding = author_binding_1.get_inner();
        let galleries_binding_1 = args.galleries.get_output(context);
        let galleries_binding = galleries_binding_1.get_inner();
        let localized_binding_1 = args.localized.get_output(context);
        let localized_binding = localized_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let template_data_binding_1 = args.template_data.get_output(context);
        let template_data_binding = template_data_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appinsights/workbookTemplate:WorkbookTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkbookTemplateResult {
            author: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("author"),
            ),
            galleries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("galleries"),
            ),
            localized: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localized"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            template_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateData"),
            ),
        }
    }
}
