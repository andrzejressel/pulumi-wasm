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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDiagnosticCategoriesArgs,
    ) -> GetDiagnosticCategoriesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getDiagnosticCategories:getDiagnosticCategories"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDiagnosticCategoriesResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            log_category_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logCategoryGroups"),
            ),
            log_category_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logCategoryTypes"),
            ),
            metrics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metrics"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
        }
    }
}
