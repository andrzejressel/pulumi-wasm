/// Manages a Log Analytics (formally Operational Insights) Windows Performance Counter DataSource.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-law")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleDataSourceWindowsPerformanceCounter = data_source_windows_performance_counter::create(
///         "exampleDataSourceWindowsPerformanceCounter",
///         DataSourceWindowsPerformanceCounterArgs::builder()
///             .counter_name("CPU")
///             .instance_name("*")
///             .interval_seconds(10)
///             .name("example-lad-wpc")
///             .object_name("CPU")
///             .resource_group_name("${example.name}")
///             .workspace_name("${exampleAnalyticsWorkspace.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Windows Performance Counter DataSources can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/dataSourceWindowsPerformanceCounter:DataSourceWindowsPerformanceCounter example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/dataSources/datasource1
/// ```
///
pub mod data_source_windows_performance_counter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceWindowsPerformanceCounterArgs {
        /// The friendly name of the performance counter.
        #[builder(into)]
        pub counter_name: pulumi_wasm_rust::Output<String>,
        /// The name of the virtual machine instance to which the Windows Performance Counter DataSource be applied. Specify a `*` will apply to all instances.
        #[builder(into)]
        pub instance_name: pulumi_wasm_rust::Output<String>,
        /// The time of sample interval in seconds. Supports values between 10 and 2147483647.
        #[builder(into)]
        pub interval_seconds: pulumi_wasm_rust::Output<i32>,
        /// The Name which should be used for this Log Analytics Windows Performance Counter DataSource. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The object name of the Log Analytics Windows Performance Counter DataSource.
        #[builder(into)]
        pub object_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        #[builder(into)]
        pub workspace_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceWindowsPerformanceCounterResult {
        /// The friendly name of the performance counter.
        pub counter_name: pulumi_wasm_rust::Output<String>,
        /// The name of the virtual machine instance to which the Windows Performance Counter DataSource be applied. Specify a `*` will apply to all instances.
        pub instance_name: pulumi_wasm_rust::Output<String>,
        /// The time of sample interval in seconds. Supports values between 10 and 2147483647.
        pub interval_seconds: pulumi_wasm_rust::Output<i32>,
        /// The Name which should be used for this Log Analytics Windows Performance Counter DataSource. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The object name of the Log Analytics Windows Performance Counter DataSource.
        pub object_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        pub workspace_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DataSourceWindowsPerformanceCounterArgs,
    ) -> DataSourceWindowsPerformanceCounterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let counter_name_binding = args.counter_name.get_inner();
        let instance_name_binding = args.instance_name.get_inner();
        let interval_seconds_binding = args.interval_seconds.get_inner();
        let name_binding = args.name.get_inner();
        let object_name_binding = args.object_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let workspace_name_binding = args.workspace_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/dataSourceWindowsPerformanceCounter:DataSourceWindowsPerformanceCounter"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "counterName".into(),
                    value: &counter_name_binding,
                },
                register_interface::ObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "intervalSeconds".into(),
                    value: &interval_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "objectName".into(),
                    value: &object_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceName".into(),
                    value: &workspace_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "counterName".into(),
                },
                register_interface::ResultField {
                    name: "instanceName".into(),
                },
                register_interface::ResultField {
                    name: "intervalSeconds".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "objectName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "workspaceName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataSourceWindowsPerformanceCounterResult {
            counter_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("counterName").unwrap(),
            ),
            instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceName").unwrap(),
            ),
            interval_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("intervalSeconds").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            object_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            workspace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceName").unwrap(),
            ),
        }
    }
}