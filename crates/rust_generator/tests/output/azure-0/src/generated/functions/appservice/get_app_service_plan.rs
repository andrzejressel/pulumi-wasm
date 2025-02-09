#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_app_service_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppServicePlanArgs {
        /// The name of the App Service Plan.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the App Service Plan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAppServicePlanResult {
        /// The ID of the App Service Environment where the App Service Plan is located.
        pub app_service_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A flag that indicates if it's a xenon plan (support for Windows Container)
        pub is_xenon: pulumi_gestalt_rust::Output<bool>,
        /// The Operating System type of the App Service Plan
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the App Service Plan exists
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of total workers allowed for this ElasticScaleEnabled App Service Plan.
        pub maximum_elastic_worker_count: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of workers supported with the App Service Plan's sku.
        pub maximum_number_of_workers: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Can Apps assigned to this App Service Plan be scaled independently?
        pub per_site_scaling: pulumi_gestalt_rust::Output<bool>,
        /// Is this App Service Plan `Reserved`?
        pub reserved: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as documented below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::super::types::appservice::GetAppServicePlanSku,
        >,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// App Service Plan perform availability zone balancing.
        pub zone_redundant: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAppServicePlanArgs,
    ) -> GetAppServicePlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getAppServicePlan:getAppServicePlan".into(),
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
        GetAppServicePlanResult {
            app_service_environment_id: o.get_field("appServiceEnvironmentId"),
            id: o.get_field("id"),
            is_xenon: o.get_field("isXenon"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            maximum_elastic_worker_count: o.get_field("maximumElasticWorkerCount"),
            maximum_number_of_workers: o.get_field("maximumNumberOfWorkers"),
            name: o.get_field("name"),
            per_site_scaling: o.get_field("perSiteScaling"),
            reserved: o.get_field("reserved"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            zone_redundant: o.get_field("zoneRedundant"),
        }
    }
}
