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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MonitorArgs,
    ) -> MonitorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_creation_source_binding = args
            .account_creation_source
            .get_output(context);
        let account_id_binding = args.account_id.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let ingestion_key_binding = args.ingestion_key.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let org_creation_source_binding = args.org_creation_source.get_output(context);
        let organization_id_binding = args.organization_id.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let user_binding = args.user.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:newrelic/monitor:Monitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountCreationSource".into(),
                    value: account_creation_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingestionKey".into(),
                    value: ingestion_key_binding.get_id(),
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
                    name: "orgCreationSource".into(),
                    value: org_creation_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationId".into(),
                    value: organization_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: user_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitorResult {
            account_creation_source: o.get_field("accountCreationSource"),
            account_id: o.get_field("accountId"),
            identity: o.get_field("identity"),
            ingestion_key: o.get_field("ingestionKey"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            org_creation_source: o.get_field("orgCreationSource"),
            organization_id: o.get_field("organizationId"),
            plan: o.get_field("plan"),
            resource_group_name: o.get_field("resourceGroupName"),
            user: o.get_field("user"),
            user_id: o.get_field("userId"),
        }
    }
}
