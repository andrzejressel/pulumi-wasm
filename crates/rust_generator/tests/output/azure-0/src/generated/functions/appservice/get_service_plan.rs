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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServicePlanArgs,
    ) -> GetServicePlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getServicePlan:getServicePlan".into(),
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
        GetServicePlanResult {
            app_service_environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceEnvironmentId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maximum_elastic_worker_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maximumElasticWorkerCount"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            os_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osType"),
            ),
            per_site_scaling_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("perSiteScalingEnabled"),
            ),
            reserved: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reserved"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            worker_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workerCount"),
            ),
            zone_balancing_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneBalancingEnabled"),
            ),
        }
    }
}
