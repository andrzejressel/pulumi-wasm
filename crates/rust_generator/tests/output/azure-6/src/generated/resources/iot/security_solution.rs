/// Manages an iot security solution.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: example-IoTHub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleSecuritySolution:
///     type: azure:iot:SecuritySolution
///     name: example
///     properties:
///       name: example-Iot-Security-Solution
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       displayName: Iot Security Solution
///       iothubIds:
///         - ${exampleIoTHub.id}
/// ```
///
/// ## Import
///
/// Iot Security Solution can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/securitySolution:SecuritySolution example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Security/iotSecuritySolutions/solution1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod security_solution {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecuritySolutionArgs {
        /// A `additional_workspace` block as defined below.
        #[builder(into, default)]
        pub additional_workspaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::SecuritySolutionAdditionalWorkspace>>,
        >,
        /// A list of disabled data sources for the Iot Security Solution. Possible value is `TwinData`.
        #[builder(into, default)]
        pub disabled_data_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the Display Name for this Iot Security Solution.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the Iot Security Solution enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of data which is to exported to analytic workspace. Valid values include `RawEvents`.
        #[builder(into, default)]
        pub events_to_exports: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the IoT Hub resource IDs to which this Iot Security Solution is applied.
        #[builder(into)]
        pub iothub_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Log Analytics Workspace ID to which the security data will be sent.
        #[builder(into, default)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Should IP addressed be unmasked in the log? Defaults to `false`.
        #[builder(into, default)]
        pub log_unmasked_ips_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Iot Security Solution. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An Azure Resource Graph query used to set the resources monitored.
        #[builder(into, default)]
        pub query_for_resources: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of subscription Ids on which the user defined resources query should be executed.
        #[builder(into, default)]
        pub query_subscription_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A `recommendations_enabled` block of options to enable or disable as defined below.
        #[builder(into, default)]
        pub recommendations_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::SecuritySolutionRecommendationsEnabled>,
        >,
        /// Specifies the name of the resource group in which to create the Iot Security Solution. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SecuritySolutionResult {
        /// A `additional_workspace` block as defined below.
        pub additional_workspaces: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::iot::SecuritySolutionAdditionalWorkspace>>,
        >,
        /// A list of disabled data sources for the Iot Security Solution. Possible value is `TwinData`.
        pub disabled_data_sources: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the Display Name for this Iot Security Solution.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Is the Iot Security Solution enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of data which is to exported to analytic workspace. Valid values include `RawEvents`.
        pub events_to_exports: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the IoT Hub resource IDs to which this Iot Security Solution is applied.
        pub iothub_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Log Analytics Workspace ID to which the security data will be sent.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should IP addressed be unmasked in the log? Defaults to `false`.
        pub log_unmasked_ips_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Iot Security Solution. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An Azure Resource Graph query used to set the resources monitored.
        pub query_for_resources: pulumi_gestalt_rust::Output<String>,
        /// A list of subscription Ids on which the user defined resources query should be executed.
        pub query_subscription_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A `recommendations_enabled` block of options to enable or disable as defined below.
        pub recommendations_enabled: pulumi_gestalt_rust::Output<
            super::super::types::iot::SecuritySolutionRecommendationsEnabled,
        >,
        /// Specifies the name of the resource group in which to create the Iot Security Solution. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SecuritySolutionArgs,
    ) -> SecuritySolutionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_workspaces_binding = args
            .additional_workspaces
            .get_output(context)
            .get_inner();
        let disabled_data_sources_binding = args
            .disabled_data_sources
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let events_to_exports_binding = args
            .events_to_exports
            .get_output(context)
            .get_inner();
        let iothub_ids_binding = args.iothub_ids.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let log_unmasked_ips_enabled_binding = args
            .log_unmasked_ips_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let query_for_resources_binding = args
            .query_for_resources
            .get_output(context)
            .get_inner();
        let query_subscription_ids_binding = args
            .query_subscription_ids
            .get_output(context)
            .get_inner();
        let recommendations_enabled_binding = args
            .recommendations_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/securitySolution:SecuritySolution".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalWorkspaces".into(),
                    value: &additional_workspaces_binding,
                },
                register_interface::ObjectField {
                    name: "disabledDataSources".into(),
                    value: &disabled_data_sources_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "eventsToExports".into(),
                    value: &events_to_exports_binding,
                },
                register_interface::ObjectField {
                    name: "iothubIds".into(),
                    value: &iothub_ids_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "logUnmaskedIpsEnabled".into(),
                    value: &log_unmasked_ips_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryForResources".into(),
                    value: &query_for_resources_binding,
                },
                register_interface::ObjectField {
                    name: "querySubscriptionIds".into(),
                    value: &query_subscription_ids_binding,
                },
                register_interface::ObjectField {
                    name: "recommendationsEnabled".into(),
                    value: &recommendations_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SecuritySolutionResult {
            additional_workspaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalWorkspaces"),
            ),
            disabled_data_sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabledDataSources"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            events_to_exports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventsToExports"),
            ),
            iothub_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubIds"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            log_unmasked_ips_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logUnmaskedIpsEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query_for_resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryForResources"),
            ),
            query_subscription_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("querySubscriptionIds"),
            ),
            recommendations_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recommendationsEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
