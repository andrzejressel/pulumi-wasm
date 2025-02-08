/// Manages a Log Analytics (formally Operational Insights) Windows Performance Counter DataSource.
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
#[allow(clippy::doc_lazy_continuation)]
pub mod data_source_windows_performance_counter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceWindowsPerformanceCounterArgs {
        /// The friendly name of the performance counter.
        #[builder(into)]
        pub counter_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the virtual machine instance to which the Windows Performance Counter DataSource be applied. Specify a `*` will apply to all instances.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time of sample interval in seconds. Supports values between 10 and 2147483647.
        #[builder(into)]
        pub interval_seconds: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The Name which should be used for this Log Analytics Windows Performance Counter DataSource. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The object name of the Log Analytics Windows Performance Counter DataSource.
        #[builder(into)]
        pub object_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        #[builder(into)]
        pub workspace_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceWindowsPerformanceCounterResult {
        /// The friendly name of the performance counter.
        pub counter_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the virtual machine instance to which the Windows Performance Counter DataSource be applied. Specify a `*` will apply to all instances.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// The time of sample interval in seconds. Supports values between 10 and 2147483647.
        pub interval_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The Name which should be used for this Log Analytics Windows Performance Counter DataSource. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The object name of the Log Analytics Windows Performance Counter DataSource.
        pub object_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Performance Counter DataSource should exist. Changing this forces a new Log Analytics Windows Performance Counter DataSource to be created.
        pub workspace_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DataSourceWindowsPerformanceCounterArgs,
    ) -> DataSourceWindowsPerformanceCounterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let counter_name_binding = args.counter_name.get_output(context).get_inner();
        let instance_name_binding = args.instance_name.get_output(context).get_inner();
        let interval_seconds_binding = args
            .interval_seconds
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let object_name_binding = args.object_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let workspace_name_binding = args.workspace_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/dataSourceWindowsPerformanceCounter:DataSourceWindowsPerformanceCounter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataSourceWindowsPerformanceCounterResult {
            counter_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("counterName"),
            ),
            instance_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceName"),
            ),
            interval_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("intervalSeconds"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            object_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            workspace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceName"),
            ),
        }
    }
}
