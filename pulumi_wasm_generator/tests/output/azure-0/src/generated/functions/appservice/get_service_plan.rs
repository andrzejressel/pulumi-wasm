pub mod get_service_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServicePlanArgs {
        /// The name of this Service Plan.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Service Plan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServicePlanResult {
        /// The ID of the App Service Environment this Service Plan is part of.
        pub app_service_environment_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A string representing the Kind of Service Plan.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Service Plan exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The maximum number of workers in use in an Elastic SKU Plan.
        pub maximum_elastic_worker_count: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The O/S type for the App Services hosted in this plan.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// Is Per Site Scaling be enabled?
        pub per_site_scaling_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether this is a reserved Service Plan Type. `true` if `os_type` is `Linux`, otherwise `false`.
        pub reserved: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU for the Service Plan.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Service Plan.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The number of Workers (instances) allocated.
        pub worker_count: pulumi_wasm_rust::Output<i32>,
        /// Is the Service Plan balance across Availability Zones in the region?
        pub zone_balancing_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServicePlanArgs) -> GetServicePlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getServicePlan:getServicePlan".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServiceEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maximumElasticWorkerCount".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "perSiteScalingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "reserved".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workerCount".into(),
                },
                register_interface::ResultField {
                    name: "zoneBalancingEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServicePlanResult {
            app_service_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceEnvironmentId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maximum_elastic_worker_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumElasticWorkerCount").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            per_site_scaling_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perSiteScalingEnabled").unwrap(),
            ),
            reserved: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reserved").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            worker_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerCount").unwrap(),
            ),
            zone_balancing_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneBalancingEnabled").unwrap(),
            ),
        }
    }
}
