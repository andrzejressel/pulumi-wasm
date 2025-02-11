#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_diagnostic_categories {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiagnosticCategoriesArgs {
        /// The ID of an existing Resource which Monitor Diagnostics Categories should be retrieved for.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDiagnosticCategoriesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of the supported log category groups of this resource to send to the destination.
        pub log_category_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of the supported log category types of this resource to send to the destination.
        pub log_category_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of the Metric Categories supported for this Resource.
        pub metrics: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDiagnosticCategoriesArgs,
    ) -> GetDiagnosticCategoriesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_id_binding = args.resource_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:monitoring/getDiagnosticCategories:getDiagnosticCategories"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDiagnosticCategoriesResult {
            id: o.get_field("id"),
            log_category_groups: o.get_field("logCategoryGroups"),
            log_category_types: o.get_field("logCategoryTypes"),
            metrics: o.get_field("metrics"),
            resource_id: o.get_field("resourceId"),
        }
    }
}
