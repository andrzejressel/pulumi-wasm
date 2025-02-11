/// Manages an Activity Log Alert within Azure Monitor.
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
///   main:
///     type: azure:monitoring:ActionGroup
///     properties:
///       name: example-actiongroup
///       resourceGroupName: ${example.name}
///       shortName: p0action
///       webhookReceivers:
///         - name: callmyapi
///           serviceUri: http://example.com/alert
///   toMonitor:
///     type: azure:storage:Account
///     name: to_monitor
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///   mainActivityLogAlert:
///     type: azure:monitoring:ActivityLogAlert
///     name: main
///     properties:
///       name: example-activitylogalert
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       scopes:
///         - ${example.id}
///       description: This alert will monitor a specific storage account updates.
///       criteria:
///         resourceId: ${toMonitor.id}
///         operationName: Microsoft.Storage/storageAccounts/write
///         category: Recommendation
///       actions:
///         - actionGroupId: ${main.id}
///           webhookProperties:
///             from: source
/// ```
///
/// ## Import
///
/// Activity log alerts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/activityLogAlert:ActivityLogAlert example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/activityLogAlerts/myalertname
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod activity_log_alert {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActivityLogAlertArgs {
        /// One or more `action` blocks as defined below.
        #[builder(into, default)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActivityLogAlertAction>>,
        >,
        /// A `criteria` block as defined below.
        #[builder(into)]
        pub criteria: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::monitoring::ActivityLogAlertCriteria,
        >,
        /// The description of this activity log alert.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should this Activity Log Alert be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the activity log alert rule should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the activity log alert. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the activity log alert instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Scope at which the Activity Log should be applied. A list of strings which could be a resource group , or a subscription, or a resource ID (such as a Storage Account).
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ActivityLogAlertResult {
        /// One or more `action` blocks as defined below.
        pub actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActivityLogAlertAction>>,
        >,
        /// A `criteria` block as defined below.
        pub criteria: pulumi_gestalt_rust::Output<
            super::super::types::monitoring::ActivityLogAlertCriteria,
        >,
        /// The description of this activity log alert.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should this Activity Log Alert be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the activity log alert rule should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the activity log alert. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the activity log alert instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Scope at which the Activity Log should be applied. A list of strings which could be a resource group , or a subscription, or a resource ID (such as a Storage Account).
        pub scopes: pulumi_gestalt_rust::Output<Vec<String>>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActivityLogAlertArgs,
    ) -> ActivityLogAlertResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let criteria_binding = args.criteria.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/activityLogAlert:ActivityLogAlert".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "criteria".into(),
                    value: &criteria_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActivityLogAlertResult {
            actions: o.get_field("actions"),
            criteria: o.get_field("criteria"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scopes: o.get_field("scopes"),
            tags: o.get_field("tags"),
        }
    }
}
