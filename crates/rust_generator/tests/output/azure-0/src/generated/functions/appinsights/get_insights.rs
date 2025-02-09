#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_insights {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInsightsArgs {
        /// Specifies the name of the Application Insights component.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Application Insights component is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInsightsResult {
        /// The App ID associated with this Application Insights component.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the component.
        pub application_type: pulumi_gestalt_rust::Output<String>,
        /// The connection string of the Application Insights component. (Sensitive)
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The instrumentation key of the Application Insights component.
        pub instrumentation_key: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the component exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The retention period in days.
        pub retention_in_days: pulumi_gestalt_rust::Output<i32>,
        /// Tags applied to the component.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The id of the associated Log Analytics workspace
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInsightsArgs,
    ) -> GetInsightsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appinsights/getInsights:getInsights".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInsightsResult {
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            application_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationType"),
            ),
            connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instrumentation_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instrumentationKey"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retention_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionInDays"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
