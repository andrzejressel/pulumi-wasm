/// Manages an Application Insights component.
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
///             .name("tf-test")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("tf-test-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Workspace Mode
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
///             .name("tf-test")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("workspace-test")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("tf-test-appinsights")
///             .resource_group_name("${example.name}")
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Insights instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/insights:Insights instance1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Insights/components/instance1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod insights {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InsightsArgs {
        /// Specifies the type of Application Insights to create. Valid values are `ios` for _iOS_, `java` for _Java web_, `MobileCenter` for _App Center_, `Node.JS` for _Node.js_, `other` for _General_, `phone` for _Windows Phone_, `store` for _Windows Store_ and `web` for _ASP.NET_. Please note these values are case sensitive; unmatched values are treated as _ASP.NET_ by Azure. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Application Insights component daily data volume cap in GB. Defaults to `100`.
        #[builder(into, default)]
        pub daily_data_cap_in_gb: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// Specifies if a notification email will be sent when the daily data volume cap is met. Defaults to `false`.
        #[builder(into, default)]
        pub daily_data_cap_notifications_disabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// By default the real client IP is masked as `0.0.0.0` in the logs. Use this argument to disable masking and log the real client IP. Defaults to `false`.
        #[builder(into, default)]
        pub disable_ip_masking: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Application Insights component force users to create their own storage account for profiling? Defaults to `false`.
        #[builder(into, default)]
        pub force_customer_storage_for_profiler: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Should the Application Insights component support ingestion over the Public Internet? Defaults to `true`.
        #[builder(into, default)]
        pub internet_ingestion_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the Application Insights component support querying over the Public Internet? Defaults to `true`.
        #[builder(into, default)]
        pub internet_query_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Disable Non-Azure AD based Auth. Defaults to `false`.
        #[builder(into, default)]
        pub local_authentication_disabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Application Insights component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Application Insights component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the retention period in days. Possible values are `30`, `60`, `90`, `120`, `180`, `270`, `365`, `550` or `730`. Defaults to `90`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the percentage of the data produced by the monitored application that is sampled for Application Insights telemetry. Defaults to `100`.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of a log analytics workspace resource.
        ///
        /// > **NOTE:** `workspace_id` cannot be removed after set. More details can be found at [Migrate to workspace-based Application Insights resources](https://docs.microsoft.com/azure/azure-monitor/app/convert-classic-resource#migration-process). If `workspace_id` is not specified but you encounter a diff, this might indicate a Microsoft initiated automatic migration from classic resources to workspace-based resources. If this is the case, please update `workspace_id` in the config file to the new value.
        #[builder(into, default)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InsightsResult {
        /// The App ID associated with this Application Insights component.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the type of Application Insights to create. Valid values are `ios` for _iOS_, `java` for _Java web_, `MobileCenter` for _App Center_, `Node.JS` for _Node.js_, `other` for _General_, `phone` for _Windows Phone_, `store` for _Windows Store_ and `web` for _ASP.NET_. Please note these values are case sensitive; unmatched values are treated as _ASP.NET_ by Azure. Changing this forces a new resource to be created.
        pub application_type: pulumi_gestalt_rust::Output<String>,
        /// The Connection String for this Application Insights component. (Sensitive)
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Application Insights component daily data volume cap in GB. Defaults to `100`.
        pub daily_data_cap_in_gb: pulumi_gestalt_rust::Output<Option<f64>>,
        /// Specifies if a notification email will be sent when the daily data volume cap is met. Defaults to `false`.
        pub daily_data_cap_notifications_disabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// By default the real client IP is masked as `0.0.0.0` in the logs. Use this argument to disable masking and log the real client IP. Defaults to `false`.
        pub disable_ip_masking: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Application Insights component force users to create their own storage account for profiling? Defaults to `false`.
        pub force_customer_storage_for_profiler: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The Instrumentation Key for this Application Insights component. (Sensitive)
        pub instrumentation_key: pulumi_gestalt_rust::Output<String>,
        /// Should the Application Insights component support ingestion over the Public Internet? Defaults to `true`.
        pub internet_ingestion_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Application Insights component support querying over the Public Internet? Defaults to `true`.
        pub internet_query_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Disable Non-Azure AD based Auth. Defaults to `false`.
        pub local_authentication_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Application Insights component. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Application Insights component. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the retention period in days. Possible values are `30`, `60`, `90`, `120`, `180`, `270`, `365`, `550` or `730`. Defaults to `90`.
        pub retention_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the percentage of the data produced by the monitored application that is sampled for Application Insights telemetry. Defaults to `100`.
        pub sampling_percentage: pulumi_gestalt_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of a log analytics workspace resource.
        ///
        /// > **NOTE:** `workspace_id` cannot be removed after set. More details can be found at [Migrate to workspace-based Application Insights resources](https://docs.microsoft.com/azure/azure-monitor/app/convert-classic-resource#migration-process). If `workspace_id` is not specified but you encounter a diff, this might indicate a Microsoft initiated automatic migration from classic resources to workspace-based resources. If this is the case, please update `workspace_id` in the config file to the new value.
        pub workspace_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InsightsArgs,
    ) -> InsightsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_type_binding = args.application_type.get_output(context);
        let daily_data_cap_in_gb_binding = args.daily_data_cap_in_gb.get_output(context);
        let daily_data_cap_notifications_disabled_binding = args
            .daily_data_cap_notifications_disabled
            .get_output(context);
        let disable_ip_masking_binding = args.disable_ip_masking.get_output(context);
        let force_customer_storage_for_profiler_binding = args
            .force_customer_storage_for_profiler
            .get_output(context);
        let internet_ingestion_enabled_binding = args
            .internet_ingestion_enabled
            .get_output(context);
        let internet_query_enabled_binding = args
            .internet_query_enabled
            .get_output(context);
        let local_authentication_disabled_binding = args
            .local_authentication_disabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retention_in_days_binding = args.retention_in_days.get_output(context);
        let sampling_percentage_binding = args.sampling_percentage.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appinsights/insights:Insights".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationType".into(),
                    value: application_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyDataCapInGb".into(),
                    value: daily_data_cap_in_gb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyDataCapNotificationsDisabled".into(),
                    value: daily_data_cap_notifications_disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableIpMasking".into(),
                    value: disable_ip_masking_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceCustomerStorageForProfiler".into(),
                    value: force_customer_storage_for_profiler_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetIngestionEnabled".into(),
                    value: internet_ingestion_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetQueryEnabled".into(),
                    value: internet_query_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationDisabled".into(),
                    value: local_authentication_disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionInDays".into(),
                    value: retention_in_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samplingPercentage".into(),
                    value: sampling_percentage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InsightsResult {
            app_id: o.get_field("appId"),
            application_type: o.get_field("applicationType"),
            connection_string: o.get_field("connectionString"),
            daily_data_cap_in_gb: o.get_field("dailyDataCapInGb"),
            daily_data_cap_notifications_disabled: o
                .get_field("dailyDataCapNotificationsDisabled"),
            disable_ip_masking: o.get_field("disableIpMasking"),
            force_customer_storage_for_profiler: o
                .get_field("forceCustomerStorageForProfiler"),
            instrumentation_key: o.get_field("instrumentationKey"),
            internet_ingestion_enabled: o.get_field("internetIngestionEnabled"),
            internet_query_enabled: o.get_field("internetQueryEnabled"),
            local_authentication_disabled: o.get_field("localAuthenticationDisabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_in_days: o.get_field("retentionInDays"),
            sampling_percentage: o.get_field("samplingPercentage"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
