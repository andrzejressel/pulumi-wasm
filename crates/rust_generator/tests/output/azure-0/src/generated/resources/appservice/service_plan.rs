/// Manages an App Service: Service Plan.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("P1v2")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AppServices can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/servicePlan:ServicePlan example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/serverFarms/farm1
/// ```
///
pub mod service_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePlanArgs {
        /// The ID of the App Service Environment to create this Service Plan in.
        ///
        /// > **NOTE:** Requires an Isolated SKU. Use one of `I1`, `I2`, `I3` for `azurerm_app_service_environment`, or `I1v2`, `I2v2`, `I3v2` for `azure.appservice.EnvironmentV3`
        #[builder(into, default)]
        pub app_service_environment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Azure Region where the Service Plan should exist. Changing this forces a new Service Plan to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of workers to use in an Elastic SKU Plan. Cannot be set unless using an Elastic SKU.
        #[builder(into, default)]
        pub maximum_elastic_worker_count: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name which should be used for this Service Plan. Changing this forces a new Service Plan to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The O/S type for the App Services to be hosted in this plan. Possible values include `Windows`, `Linux`, and `WindowsContainer`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub os_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should Per Site Scaling be enabled. Defaults to `false`.
        #[builder(into, default)]
        pub per_site_scaling_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the Service Plan should exist. Changing this forces a new Service Plan to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU for the plan. Possible values include `B1`, `B2`, `B3`, `D1`, `F1`, `I1`, `I2`, `I3`, `I1v2`, `I2v2`, `I3v2`, `I4v2`, `I5v2`, `I6v2`, `P1v2`, `P2v2`, `P3v2`, `P0v3`, `P1v3`, `P2v3`, `P3v3`, `P1mv3`, `P2mv3`, `P3mv3`, `P4mv3`, `P5mv3`, `S1`, `S2`, `S3`, `SHARED`, `EP1`, `EP2`, `EP3`, `FC1`, `WS1`, `WS2`, `WS3`, and `Y1`.
        ///
        /// > **NOTE:** Isolated SKUs (`I1`, `I2`, `I3`, `I1v2`, `I2v2`, and `I3v2`) can only be used with App Service Environments
        ///
        /// > **NOTE:** Elastic and Consumption SKUs (`Y1`, `FC1`, `EP1`, `EP2`, and `EP3`) are for use with Function Apps.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the AppService.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of Workers (instances) to be allocated.
        #[builder(into, default)]
        pub worker_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Should the Service Plan balance across Availability Zones in the region. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If this setting is set to `true` and the `worker_count` value is specified, it should be set to a multiple of the number of availability zones in the region. Please see the Azure documentation for the number of Availability Zones in your region.
        #[builder(into, default)]
        pub zone_balancing_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ServicePlanResult {
        /// The ID of the App Service Environment to create this Service Plan in.
        ///
        /// > **NOTE:** Requires an Isolated SKU. Use one of `I1`, `I2`, `I3` for `azurerm_app_service_environment`, or `I1v2`, `I2v2`, `I3v2` for `azure.appservice.EnvironmentV3`
        pub app_service_environment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A string representing the Kind of Service Plan.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Service Plan should exist. Changing this forces a new Service Plan to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of workers to use in an Elastic SKU Plan. Cannot be set unless using an Elastic SKU.
        pub maximum_elastic_worker_count: pulumi_gestalt_rust::Output<i32>,
        /// The name which should be used for this Service Plan. Changing this forces a new Service Plan to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The O/S type for the App Services to be hosted in this plan. Possible values include `Windows`, `Linux`, and `WindowsContainer`. Changing this forces a new resource to be created.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// Should Per Site Scaling be enabled. Defaults to `false`.
        pub per_site_scaling_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether this is a reserved Service Plan Type. `true` if `os_type` is `Linux`, otherwise `false`.
        pub reserved: pulumi_gestalt_rust::Output<bool>,
        /// The name of the Resource Group where the Service Plan should exist. Changing this forces a new Service Plan to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU for the plan. Possible values include `B1`, `B2`, `B3`, `D1`, `F1`, `I1`, `I2`, `I3`, `I1v2`, `I2v2`, `I3v2`, `I4v2`, `I5v2`, `I6v2`, `P1v2`, `P2v2`, `P3v2`, `P0v3`, `P1v3`, `P2v3`, `P3v3`, `P1mv3`, `P2mv3`, `P3mv3`, `P4mv3`, `P5mv3`, `S1`, `S2`, `S3`, `SHARED`, `EP1`, `EP2`, `EP3`, `FC1`, `WS1`, `WS2`, `WS3`, and `Y1`.
        ///
        /// > **NOTE:** Isolated SKUs (`I1`, `I2`, `I3`, `I1v2`, `I2v2`, and `I3v2`) can only be used with App Service Environments
        ///
        /// > **NOTE:** Elastic and Consumption SKUs (`Y1`, `FC1`, `EP1`, `EP2`, and `EP3`) are for use with Function Apps.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the AppService.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of Workers (instances) to be allocated.
        pub worker_count: pulumi_gestalt_rust::Output<i32>,
        /// Should the Service Plan balance across Availability Zones in the region. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If this setting is set to `true` and the `worker_count` value is specified, it should be set to a multiple of the number of availability zones in the region. Please see the Azure documentation for the number of Availability Zones in your region.
        pub zone_balancing_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServicePlanArgs,
    ) -> ServicePlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_service_environment_id_binding = args
            .app_service_environment_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let maximum_elastic_worker_count_binding = args
            .maximum_elastic_worker_count
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let os_type_binding = args.os_type.get_output(context).get_inner();
        let per_site_scaling_enabled_binding = args
            .per_site_scaling_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let worker_count_binding = args.worker_count.get_output(context).get_inner();
        let zone_balancing_enabled_binding = args
            .zone_balancing_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/servicePlan:ServicePlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceEnvironmentId".into(),
                    value: &app_service_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maximumElasticWorkerCount".into(),
                    value: &maximum_elastic_worker_count_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "perSiteScalingEnabled".into(),
                    value: &per_site_scaling_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workerCount".into(),
                    value: &worker_count_binding,
                },
                register_interface::ObjectField {
                    name: "zoneBalancingEnabled".into(),
                    value: &zone_balancing_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServicePlanResult {
            app_service_environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceEnvironmentId"),
            ),
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
