/// Manages an Application Insights component.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod insights {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InsightsArgs {
        /// Specifies the type of Application Insights to create. Valid values are `ios` for _iOS_, `java` for _Java web_, `MobileCenter` for _App Center_, `Node.JS` for _Node.js_, `other` for _General_, `phone` for _Windows Phone_, `store` for _Windows Store_ and `web` for _ASP.NET_. Please note these values are case sensitive; unmatched values are treated as _ASP.NET_ by Azure. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the Application Insights component daily data volume cap in GB. Defaults to `100`.
        #[builder(into, default)]
        pub daily_data_cap_in_gb: pulumi_wasm_rust::Output<Option<f64>>,
        /// Specifies if a notification email will be sent when the daily data volume cap is met. Defaults to `false`.
        #[builder(into, default)]
        pub daily_data_cap_notifications_disabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// By default the real client IP is masked as `0.0.0.0` in the logs. Use this argument to disable masking and log the real client IP. Defaults to `false`.
        #[builder(into, default)]
        pub disable_ip_masking: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Application Insights component force users to create their own storage account for profiling? Defaults to `false`.
        #[builder(into, default)]
        pub force_customer_storage_for_profiler: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Application Insights component support ingestion over the Public Internet? Defaults to `true`.
        #[builder(into, default)]
        pub internet_ingestion_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Application Insights component support querying over the Public Internet? Defaults to `true`.
        #[builder(into, default)]
        pub internet_query_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Disable Non-Azure AD based Auth. Defaults to `false`.
        #[builder(into, default)]
        pub local_authentication_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Application Insights component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Application Insights component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the retention period in days. Possible values are `30`, `60`, `90`, `120`, `180`, `270`, `365`, `550` or `730`. Defaults to `90`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the percentage of the data produced by the monitored application that is sampled for Application Insights telemetry. Defaults to `100`.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_wasm_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of a log analytics workspace resource.
        ///
        /// > **NOTE:** `workspace_id` cannot be removed after set. More details can be found at [Migrate to workspace-based Application Insights resources](https://docs.microsoft.com/azure/azure-monitor/app/convert-classic-resource#migration-process). If `workspace_id` is not specified but you encounter a diff, this might indicate a Microsoft initiated automatic migration from classic resources to workspace-based resources. If this is the case, please update `workspace_id` in the config file to the new value.
        #[builder(into, default)]
        pub workspace_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InsightsResult {
        /// The App ID associated with this Application Insights component.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the type of Application Insights to create. Valid values are `ios` for _iOS_, `java` for _Java web_, `MobileCenter` for _App Center_, `Node.JS` for _Node.js_, `other` for _General_, `phone` for _Windows Phone_, `store` for _Windows Store_ and `web` for _ASP.NET_. Please note these values are case sensitive; unmatched values are treated as _ASP.NET_ by Azure. Changing this forces a new resource to be created.
        pub application_type: pulumi_wasm_rust::Output<String>,
        /// The Connection String for this Application Insights component. (Sensitive)
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// Specifies the Application Insights component daily data volume cap in GB. Defaults to `100`.
        pub daily_data_cap_in_gb: pulumi_wasm_rust::Output<Option<f64>>,
        /// Specifies if a notification email will be sent when the daily data volume cap is met. Defaults to `false`.
        pub daily_data_cap_notifications_disabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// By default the real client IP is masked as `0.0.0.0` in the logs. Use this argument to disable masking and log the real client IP. Defaults to `false`.
        pub disable_ip_masking: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Application Insights component force users to create their own storage account for profiling? Defaults to `false`.
        pub force_customer_storage_for_profiler: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Instrumentation Key for this Application Insights component. (Sensitive)
        pub instrumentation_key: pulumi_wasm_rust::Output<String>,
        /// Should the Application Insights component support ingestion over the Public Internet? Defaults to `true`.
        pub internet_ingestion_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Application Insights component support querying over the Public Internet? Defaults to `true`.
        pub internet_query_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Disable Non-Azure AD based Auth. Defaults to `false`.
        pub local_authentication_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Application Insights component. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Application Insights component. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the retention period in days. Possible values are `30`, `60`, `90`, `120`, `180`, `270`, `365`, `550` or `730`. Defaults to `90`.
        pub retention_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the percentage of the data produced by the monitored application that is sampled for Application Insights telemetry. Defaults to `100`.
        pub sampling_percentage: pulumi_wasm_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of a log analytics workspace resource.
        ///
        /// > **NOTE:** `workspace_id` cannot be removed after set. More details can be found at [Migrate to workspace-based Application Insights resources](https://docs.microsoft.com/azure/azure-monitor/app/convert-classic-resource#migration-process). If `workspace_id` is not specified but you encounter a diff, this might indicate a Microsoft initiated automatic migration from classic resources to workspace-based resources. If this is the case, please update `workspace_id` in the config file to the new value.
        pub workspace_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InsightsArgs) -> InsightsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_type_binding = args.application_type.get_inner();
        let daily_data_cap_in_gb_binding = args.daily_data_cap_in_gb.get_inner();
        let daily_data_cap_notifications_disabled_binding = args
            .daily_data_cap_notifications_disabled
            .get_inner();
        let disable_ip_masking_binding = args.disable_ip_masking.get_inner();
        let force_customer_storage_for_profiler_binding = args
            .force_customer_storage_for_profiler
            .get_inner();
        let internet_ingestion_enabled_binding = args
            .internet_ingestion_enabled
            .get_inner();
        let internet_query_enabled_binding = args.internet_query_enabled.get_inner();
        let local_authentication_disabled_binding = args
            .local_authentication_disabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let retention_in_days_binding = args.retention_in_days.get_inner();
        let sampling_percentage_binding = args.sampling_percentage.get_inner();
        let tags_binding = args.tags.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appinsights/insights:Insights".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationType".into(),
                    value: &application_type_binding,
                },
                register_interface::ObjectField {
                    name: "dailyDataCapInGb".into(),
                    value: &daily_data_cap_in_gb_binding,
                },
                register_interface::ObjectField {
                    name: "dailyDataCapNotificationsDisabled".into(),
                    value: &daily_data_cap_notifications_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "disableIpMasking".into(),
                    value: &disable_ip_masking_binding,
                },
                register_interface::ObjectField {
                    name: "forceCustomerStorageForProfiler".into(),
                    value: &force_customer_storage_for_profiler_binding,
                },
                register_interface::ObjectField {
                    name: "internetIngestionEnabled".into(),
                    value: &internet_ingestion_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "internetQueryEnabled".into(),
                    value: &internet_query_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthenticationDisabled".into(),
                    value: &local_authentication_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "samplingPercentage".into(),
                    value: &sampling_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "applicationType".into(),
                },
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "dailyDataCapInGb".into(),
                },
                register_interface::ResultField {
                    name: "dailyDataCapNotificationsDisabled".into(),
                },
                register_interface::ResultField {
                    name: "disableIpMasking".into(),
                },
                register_interface::ResultField {
                    name: "forceCustomerStorageForProfiler".into(),
                },
                register_interface::ResultField {
                    name: "instrumentationKey".into(),
                },
                register_interface::ResultField {
                    name: "internetIngestionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "internetQueryEnabled".into(),
                },
                register_interface::ResultField {
                    name: "localAuthenticationDisabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionInDays".into(),
                },
                register_interface::ResultField {
                    name: "samplingPercentage".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InsightsResult {
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            application_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationType").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            daily_data_cap_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyDataCapInGb").unwrap(),
            ),
            daily_data_cap_notifications_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyDataCapNotificationsDisabled").unwrap(),
            ),
            disable_ip_masking: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableIpMasking").unwrap(),
            ),
            force_customer_storage_for_profiler: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceCustomerStorageForProfiler").unwrap(),
            ),
            instrumentation_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instrumentationKey").unwrap(),
            ),
            internet_ingestion_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetIngestionEnabled").unwrap(),
            ),
            internet_query_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetQueryEnabled").unwrap(),
            ),
            local_authentication_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthenticationDisabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionInDays").unwrap(),
            ),
            sampling_percentage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samplingPercentage").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
