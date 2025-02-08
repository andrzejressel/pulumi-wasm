/// Manages a Log Analytics Windows Event DataSource.
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
///     let exampleDataSourceWindowsEvent = data_source_windows_event::create(
///         "exampleDataSourceWindowsEvent",
///         DataSourceWindowsEventArgs::builder()
///             .event_log_name("Application")
///             .event_types(vec!["Error",])
///             .name("example-lad-wpc")
///             .resource_group_name("${example.name}")
///             .workspace_name("${exampleAnalyticsWorkspace.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Windows Event DataSources can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/dataSourceWindowsEvent:DataSourceWindowsEvent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/dataSources/datasource1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod data_source_windows_event {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceWindowsEventArgs {
        /// Specifies the name of the Windows Event Log to collect events from.
        #[builder(into)]
        pub event_log_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies an array of event types applied to the specified event log. Possible values include `Error`, `Warning` and `Information`.
        #[builder(into)]
        pub event_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name which should be used for this Log Analytics Windows Event DataSource. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        #[builder(into)]
        pub workspace_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceWindowsEventResult {
        /// Specifies the name of the Windows Event Log to collect events from.
        pub event_log_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies an array of event types applied to the specified event log. Possible values include `Error`, `Warning` and `Information`.
        pub event_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name which should be used for this Log Analytics Windows Event DataSource. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Log Analytics Workspace where the Log Analytics Windows Event DataSource should exist. Changing this forces a new Log Analytics Windows Event DataSource to be created.
        pub workspace_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DataSourceWindowsEventArgs,
    ) -> DataSourceWindowsEventResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let event_log_name_binding = args.event_log_name.get_output(context).get_inner();
        let event_types_binding = args.event_types.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let workspace_name_binding = args.workspace_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/dataSourceWindowsEvent:DataSourceWindowsEvent"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventLogName".into(),
                    value: &event_log_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventTypes".into(),
                    value: &event_types_binding,
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
                    name: "workspaceName".into(),
                    value: &workspace_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataSourceWindowsEventResult {
            event_log_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventLogName"),
            ),
            event_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventTypes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            workspace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceName"),
            ),
        }
    }
}
