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
        context: &pulumi_gestalt_rust::Context,
        args: GetInsightsArgs,
    ) -> GetInsightsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appinsights/getInsights:getInsights".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInsightsResult {
            app_id: o.get_field("appId"),
            application_type: o.get_field("applicationType"),
            connection_string: o.get_field("connectionString"),
            id: o.get_field("id"),
            instrumentation_key: o.get_field("instrumentationKey"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_in_days: o.get_field("retentionInDays"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
