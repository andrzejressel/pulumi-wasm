/// Manages a datadog Monitor.
///
/// ## Example Usage
///
/// ### Monitor creation with linking to Datadog organization
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-datadog
///       location: West US 2
///   exampleMonitor:
///     type: azure:datadog:Monitor
///     name: example
///     properties:
///       name: example-monitor
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datadogOrganization:
///         apiKey: XXXX
///         applicationKey: XXXX
///       user:
///         name: Example
///         email: abc@xyz.com
///       skuName: Linked
///       identity:
///         type: SystemAssigned
/// ```
///
/// ## Role Assignment
///
/// To enable metrics flow, perform role assignment on the identity created above. `Monitoring reader(43d0d8ad-25c7-4714-9337-8ba259a9fe05)` role is required .
///
/// ### Role assignment on the monitor created
///
/// ```yaml
/// resources:
///   example:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${primary.id}
///       roleDefinitionId: ${monitoringReader.roleDefinitionId}
///       principalId: ${exampleAzurermDatadogMonitor.identity[0].principalId}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   monitoringReader:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         name: Monitoring Reader
/// ```
///
/// ## Import
///
/// Datadog Monitors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datadog/monitor:Monitor example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Datadog/monitors/monitor1
/// ```
///
pub mod monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// A `datadog_organization` block as defined below.
        #[builder(into)]
        pub datadog_organization: pulumi_wasm_rust::Output<
            super::super::types::datadog::MonitorDatadogOrganization,
        >,
        /// A `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::datadog::MonitorIdentity>,
        >,
        /// The Azure Region where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Is monitoring enabled? Defaults to `true`.
        #[builder(into, default)]
        pub monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this sku.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Datadog Monitor.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user` block as defined below.
        #[builder(into)]
        pub user: pulumi_wasm_rust::Output<super::super::types::datadog::MonitorUser>,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// A `datadog_organization` block as defined below.
        pub datadog_organization: pulumi_wasm_rust::Output<
            super::super::types::datadog::MonitorDatadogOrganization,
        >,
        /// A `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::datadog::MonitorIdentity>,
        >,
        /// The Azure Region where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state.
        pub marketplace_subscription_status: pulumi_wasm_rust::Output<String>,
        /// Is monitoring enabled? Defaults to `true`.
        pub monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this sku.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Datadog Monitor.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user` block as defined below.
        pub user: pulumi_wasm_rust::Output<super::super::types::datadog::MonitorUser>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MonitorArgs) -> MonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let datadog_organization_binding = args.datadog_organization.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let monitoring_enabled_binding = args.monitoring_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_binding = args.user.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datadog/monitor:Monitor".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "datadogOrganization".into(),
                    value: &datadog_organization_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringEnabled".into(),
                    value: &monitoring_enabled_binding,
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
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "datadogOrganization".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "marketplaceSubscriptionStatus".into(),
                },
                register_interface::ResultField {
                    name: "monitoringEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MonitorResult {
            datadog_organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datadogOrganization").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            marketplace_subscription_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("marketplaceSubscriptionStatus").unwrap(),
            ),
            monitoring_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}
