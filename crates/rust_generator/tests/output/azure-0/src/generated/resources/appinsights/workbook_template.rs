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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkbookTemplateArgs,
    ) -> WorkbookTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let author_binding = args.author.get_output(context);
        let galleries_binding = args.galleries.get_output(context);
        let localized_binding = args.localized.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_data_binding = args.template_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appinsights/workbookTemplate:WorkbookTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "author".into(),
                    value: author_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleries".into(),
                    value: galleries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localized".into(),
                    value: localized_binding.get_id(),
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
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateData".into(),
                    value: template_data_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkbookTemplateResult {
            author: o.get_field("author"),
            galleries: o.get_field("galleries"),
            localized: o.get_field("localized"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            template_data: o.get_field("templateData"),
        }
    }
}
