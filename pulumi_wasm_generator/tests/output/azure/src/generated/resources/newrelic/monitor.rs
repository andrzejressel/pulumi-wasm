/// Manages an Azure Native New Relic Monitor.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: East US
///   exampleMonitor:
///     type: azure:newrelic:Monitor
///     name: example
///     properties:
///       name: example-nrm
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       plan:
///         effectiveDate: 2023-06-06T00:00:00Z
///       user:
///         email: user@example.com
///         firstName: Example
///         lastName: User
///         phoneNumber: '+12313803556'
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
///       roleDefinitionId: ${primary.id}${monitoringReader.id}
///       principalId: ${exampleAzurermNewRelicMonitor.identity[0].principalId}
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
/// Azure Native New Relic Monitor can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:newrelic/monitor:Monitor example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/NewRelic.Observability/monitors/monitor1
/// ```
///
pub mod monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// Specifies the source of account creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub account_creation_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the account id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `account_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::newrelic::MonitorIdentity>,
        >,
        /// Specifies the ingestion key of account. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub ingestion_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Azure Native New Relic Monitor. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the source of org creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub org_creation_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the organization id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `organization_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        #[builder(into, default)]
        pub organization_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into)]
        pub plan: pulumi_wasm_rust::Output<super::super::types::newrelic::MonitorPlan>,
        /// Specifies the name of the Resource Group where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `user` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into)]
        pub user: pulumi_wasm_rust::Output<super::super::types::newrelic::MonitorUser>,
        /// Specifies the user id. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub user_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// Specifies the source of account creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub account_creation_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the account id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `account_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::newrelic::MonitorIdentity>,
        >,
        /// Specifies the ingestion key of account. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub ingestion_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Azure Native New Relic Monitor. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the source of org creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub org_creation_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the organization id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `organization_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        pub organization_id: pulumi_wasm_rust::Output<String>,
        /// A `plan` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub plan: pulumi_wasm_rust::Output<super::super::types::newrelic::MonitorPlan>,
        /// Specifies the name of the Resource Group where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `user` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub user: pulumi_wasm_rust::Output<super::super::types::newrelic::MonitorUser>,
        /// Specifies the user id. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub user_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MonitorArgs) -> MonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_creation_source_binding = args.account_creation_source.get_inner();
        let account_id_binding = args.account_id.get_inner();
        let identity_binding = args.identity.get_inner();
        let ingestion_key_binding = args.ingestion_key.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let org_creation_source_binding = args.org_creation_source.get_inner();
        let organization_id_binding = args.organization_id.get_inner();
        let plan_binding = args.plan.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let user_binding = args.user.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:newrelic/monitor:Monitor".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountCreationSource".into(),
                    value: &account_creation_source_binding,
                },
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "ingestionKey".into(),
                    value: &ingestion_key_binding,
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
                    name: "orgCreationSource".into(),
                    value: &org_creation_source_binding,
                },
                register_interface::ObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountCreationSource".into(),
                },
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "ingestionKey".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgCreationSource".into(),
                },
                register_interface::ResultField {
                    name: "organizationId".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
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
            account_creation_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountCreationSource").unwrap(),
            ),
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            ingestion_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingestionKey").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_creation_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgCreationSource").unwrap(),
            ),
            organization_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationId").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}