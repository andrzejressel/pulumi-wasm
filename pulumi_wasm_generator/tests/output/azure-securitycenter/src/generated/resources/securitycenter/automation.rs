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
pub mod automation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomationArgs {
        /// One or more `action` blocks as defined below. An `action` tells this automation where the data is to be sent to upon being evaluated by the rules in the `source`.
        #[builder(into)]
        pub actions: pulumi_wasm_rust::Output<
            Vec<super::super::types::securitycenter::AutomationAction>,
        >,
        /// Specifies the description for the Security Center Automation.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean to enable or disable this Security Center Automation. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Security Center Automation. Changing this forces a new Security Center Automation to be created.
        ///
        /// > **NOTE:** For the automation to appear in Azure Portal correctly under Microsoft Defender for Cloud > Environment Settings > Account > Continuous Export, either `ExportToWorkspace` or `ExportToEventHub` must be used.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of scopes on which the automation logic is applied, at least one is required. Supported scopes are a subscription (in this format `/subscriptions/00000000-0000-0000-0000-000000000000`) or a resource group under that subscription (in the format `/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example`). The automation will only apply on defined scopes.
        #[builder(into)]
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more `source` blocks as defined below. A `source` defines what data types will be processed and a set of rules to filter that data.
        #[builder(into)]
        pub sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::securitycenter::AutomationSource>,
        >,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutomationResult {
        /// One or more `action` blocks as defined below. An `action` tells this automation where the data is to be sent to upon being evaluated by the rules in the `source`.
        pub actions: pulumi_wasm_rust::Output<
            Vec<super::super::types::securitycenter::AutomationAction>,
        >,
        /// Specifies the description for the Security Center Automation.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean to enable or disable this Security Center Automation. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Security Center Automation. Changing this forces a new Security Center Automation to be created.
        ///
        /// > **NOTE:** For the automation to appear in Azure Portal correctly under Microsoft Defender for Cloud > Environment Settings > Account > Continuous Export, either `ExportToWorkspace` or `ExportToEventHub` must be used.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Security Center Automation should exist. Changing this forces a new Security Center Automation to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of scopes on which the automation logic is applied, at least one is required. Supported scopes are a subscription (in this format `/subscriptions/00000000-0000-0000-0000-000000000000`) or a resource group under that subscription (in the format `/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example`). The automation will only apply on defined scopes.
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more `source` blocks as defined below. A `source` defines what data types will be processed and a set of rules to filter that data.
        pub sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::securitycenter::AutomationSource>,
        >,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AutomationArgs) -> AutomationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scopes_binding = args.scopes.get_inner();
        let sources_binding = args.sources.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/automation:Automation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
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
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "sources".into(),
                    value: &sources_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
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
                    name: "scopes".into(),
                },
                register_interface::ResultField {
                    name: "sources".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AutomationResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
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
            scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopes").unwrap(),
            ),
            sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sources").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
