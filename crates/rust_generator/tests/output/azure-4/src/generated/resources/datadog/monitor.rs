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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// A `datadog_organization` block as defined below.
        #[builder(into)]
        pub datadog_organization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datadog::MonitorDatadogOrganization,
        >,
        /// A `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datadog::MonitorIdentity>,
        >,
        /// The Azure Region where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is monitoring enabled? Defaults to `true`.
        #[builder(into, default)]
        pub monitoring_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this sku.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Datadog Monitor.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user` block as defined below.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datadog::MonitorUser,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// A `datadog_organization` block as defined below.
        pub datadog_organization: pulumi_gestalt_rust::Output<
            super::super::types::datadog::MonitorDatadogOrganization,
        >,
        /// A `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::datadog::MonitorIdentity>,
        >,
        /// The Azure Region where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state.
        pub marketplace_subscription_status: pulumi_gestalt_rust::Output<String>,
        /// Is monitoring enabled? Defaults to `true`.
        pub monitoring_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Datadog Monitor should exist. Changing this forces a new Datadog Monitor to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this sku.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Datadog Monitor.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user` block as defined below.
        pub user: pulumi_gestalt_rust::Output<super::super::types::datadog::MonitorUser>,
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
        let datadog_organization_binding = args.datadog_organization.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let monitoring_enabled_binding = args.monitoring_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datadog/monitor:Monitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datadogOrganization".into(),
                    value: datadog_organization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoringEnabled".into(),
                    value: monitoring_enabled_binding.get_id(),
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
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitorResult {
            datadog_organization: o.get_field("datadogOrganization"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            marketplace_subscription_status: o
                .get_field("marketplaceSubscriptionStatus"),
            monitoring_enabled: o.get_field("monitoringEnabled"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            user: o.get_field("user"),
        }
    }
}
