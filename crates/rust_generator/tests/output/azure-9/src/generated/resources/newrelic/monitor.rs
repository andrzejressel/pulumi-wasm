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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// Specifies the source of account creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub account_creation_source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the account id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `account_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::newrelic::MonitorIdentity>,
        >,
        /// Specifies the ingestion key of account. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub ingestion_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Azure Native New Relic Monitor. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the source of org creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub org_creation_source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the organization id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `organization_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        #[builder(into, default)]
        pub organization_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::newrelic::MonitorPlan,
        >,
        /// Specifies the name of the Resource Group where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `user` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::newrelic::MonitorUser,
        >,
        /// Specifies the user id. Changing this forces a new Azure Native New Relic Monitor to be created.
        #[builder(into, default)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// Specifies the source of account creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub account_creation_source: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the account id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `account_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::newrelic::MonitorIdentity>,
        >,
        /// Specifies the ingestion key of account. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub ingestion_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Azure Native New Relic Monitor. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the source of org creation. Possible values are `LIFTR` and `NEWRELIC`. Defaults to `LIFTR`. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub org_creation_source: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the organization id. Changing this forces a new Azure Native New Relic Monitor to be created.
        ///
        /// > **NOTE:** The value of `organization_id` must come from an Azure Native New Relic Monitor instance of another different subscription.
        pub organization_id: pulumi_gestalt_rust::Output<String>,
        /// A `plan` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub plan: pulumi_gestalt_rust::Output<
            super::super::types::newrelic::MonitorPlan,
        >,
        /// Specifies the name of the Resource Group where the Azure Native New Relic Monitor should exist. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `user` block as defined below. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub user: pulumi_gestalt_rust::Output<
            super::super::types::newrelic::MonitorUser,
        >,
        /// Specifies the user id. Changing this forces a new Azure Native New Relic Monitor to be created.
        pub user_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MonitorArgs,
    ) -> MonitorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_creation_source_binding = args
            .account_creation_source
            .get_output(context)
            .get_inner();
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let ingestion_key_binding = args.ingestion_key.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let org_creation_source_binding = args
            .org_creation_source
            .get_output(context)
            .get_inner();
        let organization_id_binding = args
            .organization_id
            .get_output(context)
            .get_inner();
        let plan_binding = args.plan.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:newrelic/monitor:Monitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MonitorResult {
            account_creation_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountCreationSource"),
            ),
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            ingestion_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingestionKey"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            org_creation_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("orgCreationSource"),
            ),
            organization_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("organizationId"),
            ),
            plan: pulumi_gestalt_rust::__private::into_domain(o.extract_field("plan")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            user: pulumi_gestalt_rust::__private::into_domain(o.extract_field("user")),
            user_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userId"),
            ),
        }
    }
}
