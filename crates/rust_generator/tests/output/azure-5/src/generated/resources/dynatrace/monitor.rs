/// Manages Dynatrace monitors.
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
///   exampleMonitor:
///     type: azure:dynatrace:Monitor
///     name: example
///     properties:
///       name: exmpledynatracemonitor
///       resourceGroupName: ${example.name}
///       location: ${test.location}
///       monitoringEnabled: true
///       marketplaceSubscriptionStatus: Active
///       identity:
///         type: SystemAssigned
///       user:
///         firstName: Alice
///         lastName: Bobab
///         email: alice@microsoft.com
///         phoneNumber: '123456'
///         country: westus
///       plan:
///         usageType: COMMITTED
///         billingCycle: MONTHLY
///         plan: azureportalintegration_privatepreview@TIDhjdtn7tfnxcy
///         effectiveDate: 2019-08-30T15:14:33Z
/// ```
///
/// ## Import
///
/// Dynatrace monitor can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dynatrace/monitor:Monitor example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Dynatrace.Observability/monitors/monitor1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// The kind of managed identity assigned to this resource.  A `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dynatrace::MonitorIdentity,
        >,
        /// The Azure Region where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state. Possible values are `Active` and `Suspended`.
        #[builder(into)]
        pub marketplace_subscription: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Flag specifying if the resource monitoring is enabled or disabled. Default is `true`.
        #[builder(into, default)]
        pub monitoring_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the Dynatrace monitor. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Billing plan information. A `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dynatrace::MonitorPlan,
        >,
        /// The name of the Resource Group where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User's information. A `user` block as defined below. Chainging this forces a new resource to be created.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dynatrace::MonitorUser,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// The kind of managed identity assigned to this resource.  A `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::dynatrace::MonitorIdentity,
        >,
        /// The Azure Region where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state. Possible values are `Active` and `Suspended`.
        pub marketplace_subscription: pulumi_gestalt_rust::Output<String>,
        /// Flag specifying if the resource monitoring is enabled or disabled. Default is `true`.
        pub monitoring_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the Dynatrace monitor. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Billing plan information. A `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<
            super::super::types::dynatrace::MonitorPlan,
        >,
        /// The name of the Resource Group where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User's information. A `user` block as defined below. Chainging this forces a new resource to be created.
        pub user: pulumi_gestalt_rust::Output<
            super::super::types::dynatrace::MonitorUser,
        >,
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
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let marketplace_subscription_binding = args
            .marketplace_subscription
            .get_output(context);
        let monitoring_enabled_binding = args.monitoring_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dynatrace/monitor:Monitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "marketplaceSubscription".into(),
                    value: &marketplace_subscription_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoringEnabled".into(),
                    value: &monitoring_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: &user_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitorResult {
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            marketplace_subscription: o.get_field("marketplaceSubscription"),
            monitoring_enabled: o.get_field("monitoringEnabled"),
            name: o.get_field("name"),
            plan: o.get_field("plan"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            user: o.get_field("user"),
        }
    }
}
