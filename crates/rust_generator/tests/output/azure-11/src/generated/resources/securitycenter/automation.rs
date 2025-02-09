/// Manages Security Center Automation and Continuous Export. This resource supports three types of destination in the `action`, Logic Apps, Log Analytics and Event Hubs
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
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       capacity: 2
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: acceptanceTestEventHub
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 2
///       messageRetention: 2
///   exampleAuthorizationRule:
///     type: azure:eventhub:AuthorizationRule
///     name: example
///     properties:
///       name: example-rule
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///       listen: true
///       send: false
///       manage: false
///   exampleAutomation:
///     type: azure:securitycenter:Automation
///     name: example
///     properties:
///       name: example-automation
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       actions:
///         - type: eventhub
///           resourceId: ${exampleEventHub.id}
///           connectionString: ${exampleAuthorizationRule.primaryConnectionString}
///       sources:
///         - eventSource: Alerts
///           ruleSets:
///             - rules:
///                 - propertyPath: properties.metadata.severity
///                   operator: Equals
///                   expectedValue: High
///                   propertyType: String
///       scopes:
///         - /subscriptions/${current.subscriptionId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Security Center Automations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/automation:Automation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Security/automations/automation1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod automation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomationArgs {
        /// One or more `action` blocks as defined below. An `action` tells this automation where the data is to be sent to upon being evaluated by the rules in the `source`.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::securitycenter::AutomationAction>,
        >,
        /// Specifies the description for the Security Center Automation.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean to enable or disable this Security Center Automation. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Security Center Automation. Changing this forces a new Security Center Automation to be created.
        ///
        /// > **NOTE:** For the automation to appear in Azure Portal correctly under Microsoft Defender for Cloud > Environment Settings > Account > Continuous Export, either `ExportToWorkspace` or `ExportToEventHub` must be used.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of scopes on which the automation logic is applied, at least one is required. Supported scopes are a subscription (in this format `/subscriptions/00000000-0000-0000-0000-000000000000`) or a resource group under that subscription (in the format `/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example`). The automation will only apply on defined scopes.
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// One or more `source` blocks as defined below. A `source` defines what data types will be processed and a set of rules to filter that data.
        #[builder(into)]
        pub sources: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::securitycenter::AutomationSource>,
        >,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutomationResult {
        /// One or more `action` blocks as defined below. An `action` tells this automation where the data is to be sent to upon being evaluated by the rules in the `source`.
        pub actions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securitycenter::AutomationAction>,
        >,
        /// Specifies the description for the Security Center Automation.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean to enable or disable this Security Center Automation. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Security Center Automation. Changing this forces a new Security Center Automation to be created.
        ///
        /// > **NOTE:** For the automation to appear in Azure Portal correctly under Microsoft Defender for Cloud > Environment Settings > Account > Continuous Export, either `ExportToWorkspace` or `ExportToEventHub` must be used.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of scopes on which the automation logic is applied, at least one is required. Supported scopes are a subscription (in this format `/subscriptions/00000000-0000-0000-0000-000000000000`) or a resource group under that subscription (in the format `/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example`). The automation will only apply on defined scopes.
        pub scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more `source` blocks as defined below. A `source` defines what data types will be processed and a set of rules to filter that data.
        pub sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securitycenter::AutomationSource>,
        >,
        /// A mapping of tags assigned to the resource.
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
        args: AutomationArgs,
    ) -> AutomationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let sources_binding = args.sources.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/automation:Automation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
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
                    name: "scopes".into(),
                    value: scopes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sources".into(),
                    value: sources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutomationResult {
            actions: o.get_field("actions"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scopes: o.get_field("scopes"),
            sources: o.get_field("sources"),
            tags: o.get_field("tags"),
        }
    }
}
