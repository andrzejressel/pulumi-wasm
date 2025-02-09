/// Manages a Cost Management Export for a Billing Account.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: examplecontainer
///       storageAccountName: ${exampleAccount.name}
///   exampleAccountCostManagementExport:
///     type: azure:billing:AccountCostManagementExport
///     name: example
///     properties:
///       name: example
///       billingAccountId: example
///       recurrenceType: Monthly
///       recurrencePeriodStartDate: 2020-08-18T00:00:00Z
///       recurrencePeriodEndDate: 2020-09-18T00:00:00Z
///       exportDataStorageLocation:
///         containerId: ${exampleContainer.resourceManagerId}
///         rootFolderPath: /root/updated
///       exportDataOptions:
///         type: Usage
///         timeFrame: WeekToDate
/// ```
///
/// ## Import
///
/// Billing Account Cost Management Exports can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:billing/accountCostManagementExport:AccountCostManagementExport example /providers/Microsoft.Billing/billingAccounts/12345678/providers/Microsoft.CostManagement/exports/export1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_cost_management_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountCostManagementExportArgs {
        /// Is the cost management export active? Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The id of the billing account on which to create an export. Changing this forces a new resource to be created.
        #[builder(into)]
        pub billing_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `export_data_options` block as defined below.
        #[builder(into)]
        pub export_data_options: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::billing::AccountCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        #[builder(into)]
        pub export_data_storage_location: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::billing::AccountCostManagementExportExportDataStorageLocation,
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
    }
    #[allow(dead_code)]
    pub struct AccountCostManagementExportResult {
        /// Is the cost management export active? Default is `true`.
        pub active: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The id of the billing account on which to create an export. Changing this forces a new resource to be created.
        pub billing_account_id: pulumi_gestalt_rust::Output<String>,
        /// A `export_data_options` block as defined below.
        pub export_data_options: pulumi_gestalt_rust::Output<
            super::super::types::billing::AccountCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        pub export_data_storage_location: pulumi_gestalt_rust::Output<
            super::super::types::billing::AccountCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The date the export will stop capturing information.
        pub recurrence_period_end_date: pulumi_gestalt_rust::Output<String>,
        /// The date the export will start capturing information.
        pub recurrence_period_start_date: pulumi_gestalt_rust::Output<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        pub recurrence_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountCostManagementExportArgs,
    ) -> AccountCostManagementExportResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let active_binding = args.active.get_output(context);
        let billing_account_id_binding = args.billing_account_id.get_output(context);
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
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:billing/accountCostManagementExport:AccountCostManagementExport"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "active".into(),
                    value: active_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAccountId".into(),
                    value: billing_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportDataOptions".into(),
                    value: export_data_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportDataStorageLocation".into(),
                    value: export_data_storage_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrencePeriodEndDate".into(),
                    value: recurrence_period_end_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrencePeriodStartDate".into(),
                    value: recurrence_period_start_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recurrenceType".into(),
                    value: recurrence_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountCostManagementExportResult {
            active: o.get_field("active"),
            billing_account_id: o.get_field("billingAccountId"),
            export_data_options: o.get_field("exportDataOptions"),
            export_data_storage_location: o.get_field("exportDataStorageLocation"),
            name: o.get_field("name"),
            recurrence_period_end_date: o.get_field("recurrencePeriodEndDate"),
            recurrence_period_start_date: o.get_field("recurrencePeriodStartDate"),
            recurrence_type: o.get_field("recurrenceType"),
        }
    }
}
