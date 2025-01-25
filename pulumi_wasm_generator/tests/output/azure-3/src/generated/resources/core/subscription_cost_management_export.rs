/// Manages a Cost Management Export for a Subscription.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: examplecontainer
///       storageAccountName: ${exampleAccount.name}
///   exampleSubscriptionCostManagementExport:
///     type: azure:core:SubscriptionCostManagementExport
///     name: example
///     properties:
///       name: example
///       subscriptionId: ${example.id}
///       recurrenceType: Monthly
///       recurrencePeriodStartDate: 2020-08-18T00:00:00Z
///       recurrencePeriodEndDate: 2020-09-18T00:00:00Z
///       exportDataStorageLocation:
///         containerId: ${exampleContainer.resourceManagerId}
///         rootFolderPath: /root/updated
///       exportDataOptions:
///         type: Usage
///         timeFrame: WeekToDate
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Subscription Cost Management Exports can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/subscriptionCostManagementExport:SubscriptionCostManagementExport example /subscriptions/12345678-1234-9876-4563-123456789012/providers/Microsoft.CostManagement/exports/export1
/// ```
///
pub mod subscription_cost_management_export {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionCostManagementExportArgs {
        /// Is the cost management export active? Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `export_data_options` block as defined below.
        #[builder(into)]
        pub export_data_options: pulumi_wasm_rust::InputOrOutput<
            super::super::types::core::SubscriptionCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        #[builder(into)]
        pub export_data_storage_location: pulumi_wasm_rust::InputOrOutput<
            super::super::types::core::SubscriptionCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The date the export will stop capturing information.
        #[builder(into)]
        pub recurrence_period_end_date: pulumi_wasm_rust::InputOrOutput<String>,
        /// The date the export will start capturing information.
        #[builder(into)]
        pub recurrence_period_start_date: pulumi_wasm_rust::InputOrOutput<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        #[builder(into)]
        pub recurrence_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the subscription on which to create an export. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subscription_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionCostManagementExportResult {
        /// Is the cost management export active? Default is `true`.
        pub active: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `export_data_options` block as defined below.
        pub export_data_options: pulumi_wasm_rust::Output<
            super::super::types::core::SubscriptionCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        pub export_data_storage_location: pulumi_wasm_rust::Output<
            super::super::types::core::SubscriptionCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The date the export will stop capturing information.
        pub recurrence_period_end_date: pulumi_wasm_rust::Output<String>,
        /// The date the export will start capturing information.
        pub recurrence_period_start_date: pulumi_wasm_rust::Output<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        pub recurrence_type: pulumi_wasm_rust::Output<String>,
        /// The id of the subscription on which to create an export. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubscriptionCostManagementExportArgs,
    ) -> SubscriptionCostManagementExportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_binding = args.active.get_output(context).get_inner();
        let export_data_options_binding = args
            .export_data_options
            .get_output(context)
            .get_inner();
        let export_data_storage_location_binding = args
            .export_data_storage_location
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let recurrence_period_end_date_binding = args
            .recurrence_period_end_date
            .get_output(context)
            .get_inner();
        let recurrence_period_start_date_binding = args
            .recurrence_period_start_date
            .get_output(context)
            .get_inner();
        let recurrence_type_binding = args
            .recurrence_type
            .get_output(context)
            .get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/subscriptionCostManagementExport:SubscriptionCostManagementExport"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "exportDataOptions".into(),
                    value: &export_data_options_binding,
                },
                register_interface::ObjectField {
                    name: "exportDataStorageLocation".into(),
                    value: &export_data_storage_location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recurrencePeriodEndDate".into(),
                    value: &recurrence_period_end_date_binding,
                },
                register_interface::ObjectField {
                    name: "recurrencePeriodStartDate".into(),
                    value: &recurrence_period_start_date_binding,
                },
                register_interface::ObjectField {
                    name: "recurrenceType".into(),
                    value: &recurrence_type_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "active".into(),
                },
                register_interface::ResultField {
                    name: "exportDataOptions".into(),
                },
                register_interface::ResultField {
                    name: "exportDataStorageLocation".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recurrencePeriodEndDate".into(),
                },
                register_interface::ResultField {
                    name: "recurrencePeriodStartDate".into(),
                },
                register_interface::ResultField {
                    name: "recurrenceType".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubscriptionCostManagementExportResult {
            active: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("active").unwrap(),
            ),
            export_data_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportDataOptions").unwrap(),
            ),
            export_data_storage_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportDataStorageLocation").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recurrence_period_end_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurrencePeriodEndDate").unwrap(),
            ),
            recurrence_period_start_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurrencePeriodStartDate").unwrap(),
            ),
            recurrence_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurrenceType").unwrap(),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionId").unwrap(),
            ),
        }
    }
}
