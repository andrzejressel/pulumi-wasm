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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription_cost_management_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionCostManagementExportArgs {
        /// Is the cost management export active? Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `export_data_options` block as defined below.
        #[builder(into)]
        pub export_data_options: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::core::SubscriptionCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        #[builder(into)]
        pub export_data_storage_location: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::core::SubscriptionCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The date the export will stop capturing information.
        #[builder(into)]
        pub recurrence_period_end_date: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The date the export will start capturing information.
        #[builder(into)]
        pub recurrence_period_start_date: pulumi_gestalt_rust::InputOrOutput<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        #[builder(into)]
        pub recurrence_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the subscription on which to create an export. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionCostManagementExportResult {
        /// Is the cost management export active? Default is `true`.
        pub active: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `export_data_options` block as defined below.
        pub export_data_options: pulumi_gestalt_rust::Output<
            super::super::types::core::SubscriptionCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        pub export_data_storage_location: pulumi_gestalt_rust::Output<
            super::super::types::core::SubscriptionCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The date the export will stop capturing information.
        pub recurrence_period_end_date: pulumi_gestalt_rust::Output<String>,
        /// The date the export will start capturing information.
        pub recurrence_period_start_date: pulumi_gestalt_rust::Output<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        pub recurrence_type: pulumi_gestalt_rust::Output<String>,
        /// The id of the subscription on which to create an export. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriptionCostManagementExportArgs,
    ) -> SubscriptionCostManagementExportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let active_binding = args.active.get_output(context);
        let export_data_options_binding = args.export_data_options.get_output(context);
        let export_data_storage_location_binding = args
            .export_data_storage_location
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let recurrence_period_end_date_binding = args
            .recurrence_period_end_date
            .get_output(context);
        let recurrence_period_start_date_binding = args
            .recurrence_period_start_date
            .get_output(context);
        let recurrence_type_binding = args.recurrence_type.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/subscriptionCostManagementExport:SubscriptionCostManagementExport"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "active".into(),
                    value: &active_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportDataOptions".into(),
                    value: &export_data_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportDataStorageLocation".into(),
                    value: &export_data_storage_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrencePeriodEndDate".into(),
                    value: &recurrence_period_end_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrencePeriodStartDate".into(),
                    value: &recurrence_period_start_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrenceType".into(),
                    value: &recurrence_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriptionCostManagementExportResult {
            active: o.get_field("active"),
            export_data_options: o.get_field("exportDataOptions"),
            export_data_storage_location: o.get_field("exportDataStorageLocation"),
            name: o.get_field("name"),
            recurrence_period_end_date: o.get_field("recurrencePeriodEndDate"),
            recurrence_period_start_date: o.get_field("recurrencePeriodStartDate"),
            recurrence_type: o.get_field("recurrenceType"),
            subscription_id: o.get_field("subscriptionId"),
        }
    }
}
