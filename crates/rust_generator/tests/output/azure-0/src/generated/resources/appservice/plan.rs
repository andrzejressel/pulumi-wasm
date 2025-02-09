/// Manages an App Service Plan component.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.ServicePlan` resource instead.
///
/// ## Example Usage
///
/// ### Dedicated)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .location("${example.location}")
///             .name("api-appserviceplan-pro")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Shared / Consumption Plan)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .kind("FunctionApp")
///             .location("${example.location}")
///             .name("api-appserviceplan-pro")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("Y1").tier("Dynamic").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Linux)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .kind("Linux")
///             .location("${example.location}")
///             .name("api-appserviceplan-pro")
///             .reserved(true)
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Windows Container)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .is_xenon(true)
///             .kind("xenon")
///             .location("${example.location}")
///             .name("api-appserviceplan-pro")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("PC2").tier("PremiumContainer").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Plan instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/plan:Plan instance1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/serverFarms/instance1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlanArgs {
        /// The ID of the App Service Environment where the App Service Plan should be located. Changing forces a new resource to be created.
        ///
        /// > **NOTE:** Attaching to an App Service Environment requires the App Service Plan use a `Premium` SKU (when using an ASEv1) and the `Isolated` SKU (for an ASEv2).
        #[builder(into, default)]
        pub app_service_environment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether to create a xenon App Service Plan.
        #[builder(into, default)]
        pub is_xenon: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The kind of the App Service Plan to create. Possible values are `Windows` (also available as `App`), `Linux`, `elastic` (for Premium Consumption), `xenon` and `FunctionApp` (for a Consumption Plan). Defaults to `Windows`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When creating a `Linux` App Service Plan, the `reserved` field must be set to `true`, and when creating a `Windows`/`app` App Service Plan the `reserved` field must be set to `false`.
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of total workers allowed for this ElasticScaleEnabled App Service Plan.
        #[builder(into, default)]
        pub maximum_elastic_worker_count: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Specifies the name of the App Service Plan component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Can Apps assigned to this App Service Plan be scaled independently? If set to `false` apps assigned to this plan will scale to all instances of the plan.
        #[builder(into, default)]
        pub per_site_scaling: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is this App Service Plan `Reserved`.
        #[builder(into, default)]
        pub reserved: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group in which to create the App Service Plan component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sku` block as documented below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appservice::PlanSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if the App Service Plan should be Zone Redundant. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Requires either `PremiumV2` or `PremiumV3` SKU and that at least 3 instances. For more information, please see the [App Service Team Blog](https://azure.github.io/AppService/2021/08/25/App-service-support-for-availability-zones.html).
        #[builder(into, default)]
        pub zone_redundant: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct PlanResult {
        /// The ID of the App Service Environment where the App Service Plan should be located. Changing forces a new resource to be created.
        ///
        /// > **NOTE:** Attaching to an App Service Environment requires the App Service Plan use a `Premium` SKU (when using an ASEv1) and the `Isolated` SKU (for an ASEv2).
        pub app_service_environment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to create a xenon App Service Plan.
        pub is_xenon: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The kind of the App Service Plan to create. Possible values are `Windows` (also available as `App`), `Linux`, `elastic` (for Premium Consumption), `xenon` and `FunctionApp` (for a Consumption Plan). Defaults to `Windows`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When creating a `Linux` App Service Plan, the `reserved` field must be set to `true`, and when creating a `Windows`/`app` App Service Plan the `reserved` field must be set to `false`.
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of total workers allowed for this ElasticScaleEnabled App Service Plan.
        pub maximum_elastic_worker_count: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of workers supported with the App Service Plan's sku.
        pub maximum_number_of_workers: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the App Service Plan component. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Can Apps assigned to this App Service Plan be scaled independently? If set to `false` apps assigned to this plan will scale to all instances of the plan.
        pub per_site_scaling: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is this App Service Plan `Reserved`.
        pub reserved: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the App Service Plan component. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as documented below.
        pub sku: pulumi_gestalt_rust::Output<super::super::types::appservice::PlanSku>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if the App Service Plan should be Zone Redundant. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Requires either `PremiumV2` or `PremiumV3` SKU and that at least 3 instances. For more information, please see the [App Service Team Blog](https://azure.github.io/AppService/2021/08/25/App-service-support-for-availability-zones.html).
        pub zone_redundant: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PlanArgs,
    ) -> PlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_service_environment_id_binding = args
            .app_service_environment_id
            .get_output(context);
        let is_xenon_binding = args.is_xenon.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let location_binding = args.location.get_output(context);
        let maximum_elastic_worker_count_binding = args
            .maximum_elastic_worker_count
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let per_site_scaling_binding = args.per_site_scaling.get_output(context);
        let reserved_binding = args.reserved.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zone_redundant_binding = args.zone_redundant.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/plan:Plan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServiceEnvironmentId".into(),
                    value: app_service_environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isXenon".into(),
                    value: is_xenon_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumElasticWorkerCount".into(),
                    value: maximum_elastic_worker_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "perSiteScaling".into(),
                    value: per_site_scaling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reserved".into(),
                    value: reserved_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneRedundant".into(),
                    value: zone_redundant_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PlanResult {
            app_service_environment_id: o.get_field("appServiceEnvironmentId"),
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
