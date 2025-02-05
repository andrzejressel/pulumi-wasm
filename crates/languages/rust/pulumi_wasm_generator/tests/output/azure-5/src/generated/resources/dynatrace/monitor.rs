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
pub mod monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorArgs {
        /// The kind of managed identity assigned to this resource.  A `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            super::super::types::dynatrace::MonitorIdentity,
        >,
        /// The Azure Region where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state. Possible values are `Active` and `Suspended`.
        #[builder(into)]
        pub marketplace_subscription: pulumi_wasm_rust::InputOrOutput<String>,
        /// Flag specifying if the resource monitoring is enabled or disabled. Default is `true`.
        #[builder(into, default)]
        pub monitoring_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the Dynatrace monitor. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Billing plan information. A `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub plan: pulumi_wasm_rust::InputOrOutput<
            super::super::types::dynatrace::MonitorPlan,
        >,
        /// The name of the Resource Group where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User's information. A `user` block as defined below. Chainging this forces a new resource to be created.
        #[builder(into)]
        pub user: pulumi_wasm_rust::InputOrOutput<
            super::super::types::dynatrace::MonitorUser,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitorResult {
        /// The kind of managed identity assigned to this resource.  A `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::dynatrace::MonitorIdentity,
        >,
        /// The Azure Region where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state. Possible values are `Active` and `Suspended`.
        pub marketplace_subscription: pulumi_wasm_rust::Output<String>,
        /// Flag specifying if the resource monitoring is enabled or disabled. Default is `true`.
        pub monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the Dynatrace monitor. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Billing plan information. A `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_wasm_rust::Output<super::super::types::dynatrace::MonitorPlan>,
        /// The name of the Resource Group where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User's information. A `user` block as defined below. Chainging this forces a new resource to be created.
        pub user: pulumi_wasm_rust::Output<super::super::types::dynatrace::MonitorUser>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MonitorArgs,
    ) -> MonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let marketplace_subscription_binding = args
            .marketplace_subscription
            .get_output(context)
            .get_inner();
        let monitoring_enabled_binding = args
            .monitoring_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let plan_binding = args.plan.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dynatrace/monitor:Monitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "marketplaceSubscription".into(),
                    value: &marketplace_subscription_binding,
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
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MonitorResult {
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            marketplace_subscription: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("marketplaceSubscription"),
            ),
            monitoring_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("monitoringEnabled"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            plan: pulumi_wasm_rust::__private::into_domain(o.extract_field("plan")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            user: pulumi_wasm_rust::__private::into_domain(o.extract_field("user")),
        }
    }
}
