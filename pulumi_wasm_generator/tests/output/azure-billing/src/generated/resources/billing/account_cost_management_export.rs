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
pub mod account_cost_management_export {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountCostManagementExportArgs {
        /// Is the cost management export active? Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_wasm_rust::Output<Option<bool>>,
        /// The id of the billing account on which to create an export. Changing this forces a new resource to be created.
        #[builder(into)]
        pub billing_account_id: pulumi_wasm_rust::Output<String>,
        /// A `export_data_options` block as defined below.
        #[builder(into)]
        pub export_data_options: pulumi_wasm_rust::Output<
            super::super::types::billing::AccountCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        #[builder(into)]
        pub export_data_storage_location: pulumi_wasm_rust::Output<
            super::super::types::billing::AccountCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The date the export will stop capturing information.
        #[builder(into)]
        pub recurrence_period_end_date: pulumi_wasm_rust::Output<String>,
        /// The date the export will start capturing information.
        #[builder(into)]
        pub recurrence_period_start_date: pulumi_wasm_rust::Output<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        #[builder(into)]
        pub recurrence_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccountCostManagementExportResult {
        /// Is the cost management export active? Default is `true`.
        pub active: pulumi_wasm_rust::Output<Option<bool>>,
        /// The id of the billing account on which to create an export. Changing this forces a new resource to be created.
        pub billing_account_id: pulumi_wasm_rust::Output<String>,
        /// A `export_data_options` block as defined below.
        pub export_data_options: pulumi_wasm_rust::Output<
            super::super::types::billing::AccountCostManagementExportExportDataOptions,
        >,
        /// A `export_data_storage_location` block as defined below.
        pub export_data_storage_location: pulumi_wasm_rust::Output<
            super::super::types::billing::AccountCostManagementExportExportDataStorageLocation,
        >,
        /// Specifies the name of the Cost Management Export. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The date the export will stop capturing information.
        pub recurrence_period_end_date: pulumi_wasm_rust::Output<String>,
        /// The date the export will start capturing information.
        pub recurrence_period_start_date: pulumi_wasm_rust::Output<String>,
        /// How often the requested information will be exported. Valid values include `Annually`, `Daily`, `Monthly`, `Weekly`.
        pub recurrence_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccountCostManagementExportArgs,
    ) -> AccountCostManagementExportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_binding = args.active.get_inner();
        let billing_account_id_binding = args.billing_account_id.get_inner();
        let export_data_options_binding = args.export_data_options.get_inner();
        let export_data_storage_location_binding = args
            .export_data_storage_location
            .get_inner();
        let name_binding = args.name.get_inner();
        let recurrence_period_end_date_binding = args
            .recurrence_period_end_date
            .get_inner();
        let recurrence_period_start_date_binding = args
            .recurrence_period_start_date
            .get_inner();
        let recurrence_type_binding = args.recurrence_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:billing/accountCostManagementExport:AccountCostManagementExport"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "billingAccountId".into(),
                    value: &billing_account_id_binding,
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "active".into(),
                },
                register_interface::ResultField {
                    name: "billingAccountId".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountCostManagementExportResult {
            active: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("active").unwrap(),
            ),
            billing_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAccountId").unwrap(),
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
        }
    }
}