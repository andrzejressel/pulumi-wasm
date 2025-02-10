#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServicePlanArgs {
        /// The name of this Service Plan.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Service Plan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServicePlanResult {
        /// The ID of the App Service Environment this Service Plan is part of.
        pub app_service_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A string representing the Kind of Service Plan.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Service Plan exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of workers in use in an Elastic SKU Plan.
        pub maximum_elastic_worker_count: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The O/S type for the App Services hosted in this plan.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// Is Per Site Scaling be enabled?
        pub per_site_scaling_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether this is a reserved Service Plan Type. `true` if `os_type` is `Linux`, otherwise `false`.
        pub reserved: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU for the Service Plan.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Service Plan.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The number of Workers (instances) allocated.
        pub worker_count: pulumi_gestalt_rust::Output<i32>,
        /// Is the Service Plan balance across Availability Zones in the region?
        pub zone_balancing_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServicePlanArgs,
    ) -> GetServicePlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getServicePlan:getServicePlan".into(),
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
        GetServicePlanResult {
            app_service_environment_id: o.get_field("appServiceEnvironmentId"),
            id: o.get_field("id"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            maximum_elastic_worker_count: o.get_field("maximumElasticWorkerCount"),
            name: o.get_field("name"),
            os_type: o.get_field("osType"),
            per_site_scaling_enabled: o.get_field("perSiteScalingEnabled"),
            reserved: o.get_field("reserved"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            worker_count: o.get_field("workerCount"),
            zone_balancing_enabled: o.get_field("zoneBalancingEnabled"),
        }
    }
}
