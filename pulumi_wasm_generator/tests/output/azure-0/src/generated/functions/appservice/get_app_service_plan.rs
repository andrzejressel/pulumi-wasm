pub mod get_app_service_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppServicePlanArgs {
        /// The name of the App Service Plan.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the App Service Plan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAppServicePlanResult {
        /// The ID of the App Service Environment where the App Service Plan is located.
        pub app_service_environment_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A flag that indicates if it's a xenon plan (support for Windows Container)
        pub is_xenon: pulumi_wasm_rust::Output<bool>,
        /// The Operating System type of the App Service Plan
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure location where the App Service Plan exists
        pub location: pulumi_wasm_rust::Output<String>,
        /// The maximum number of total workers allowed for this ElasticScaleEnabled App Service Plan.
        pub maximum_elastic_worker_count: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of workers supported with the App Service Plan's sku.
        pub maximum_number_of_workers: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Can Apps assigned to this App Service Plan be scaled independently?
        pub per_site_scaling: pulumi_wasm_rust::Output<bool>,
        /// Is this App Service Plan `Reserved`?
        pub reserved: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as documented below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::super::types::appservice::GetAppServicePlanSku,
        >,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// App Service Plan perform availability zone balancing.
        pub zone_redundant: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAppServicePlanArgs,
    ) -> GetAppServicePlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getAppServicePlan:getAppServicePlan".into(),
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
        GetAppServicePlanResult {
            app_service_environment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appServiceEnvironmentId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            is_xenon: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isXenon"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maximum_elastic_worker_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maximumElasticWorkerCount"),
            ),
            maximum_number_of_workers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maximumNumberOfWorkers"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            per_site_scaling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("perSiteScaling"),
            ),
            reserved: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reserved"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            zone_redundant: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("zoneRedundant"),
            ),
        }
    }
}
