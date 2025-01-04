/// Manages a Logic App Integration Account Batch Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleIntegrationAccount = integration_account::create(
///         "exampleIntegrationAccount",
///         IntegrationAccountArgs::builder()
///             .location("${example.location}")
///             .name("example-ia")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleIntegrationAccountBatchConfiguration = integration_account_batch_configuration::create(
///         "exampleIntegrationAccountBatchConfiguration",
///         IntegrationAccountBatchConfigurationArgs::builder()
///             .batch_group_name("TestBatchGroup")
///             .integration_account_name("${exampleIntegrationAccount.name}")
///             .name("exampleiabc")
///             .release_criteria(
///                 IntegrationAccountBatchConfigurationReleaseCriteria::builder()
///                     .messageCount(80)
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Integration Account Batch Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountBatchConfiguration:IntegrationAccountBatchConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/batchConfigurations/batchConfiguration1
/// ```
///
pub mod integration_account_batch_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountBatchConfigurationArgs {
        /// The batch group name of the Logic App Integration Batch Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub batch_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Batch Configuration.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Batch Configuration. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `release_criteria` block as documented below, which is used to select the criteria to meet before processing each batch.
        #[builder(into)]
        pub release_criteria: pulumi_wasm_rust::Output<
            super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteria,
        >,
        /// The name of the Resource Group where the Logic App Integration Account Batch Configuration should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountBatchConfigurationResult {
        /// The batch group name of the Logic App Integration Batch Configuration. Changing this forces a new resource to be created.
        pub batch_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Batch Configuration.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Batch Configuration. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `release_criteria` block as documented below, which is used to select the criteria to meet before processing each batch.
        pub release_criteria: pulumi_wasm_rust::Output<
            super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteria,
        >,
        /// The name of the Resource Group where the Logic App Integration Account Batch Configuration should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationAccountBatchConfigurationArgs,
    ) -> IntegrationAccountBatchConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let batch_group_name_binding = args.batch_group_name.get_inner();
        let integration_account_name_binding = args.integration_account_name.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let release_criteria_binding = args.release_criteria.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountBatchConfiguration:IntegrationAccountBatchConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "batchGroupName".into(),
                    value: &batch_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "releaseCriteria".into(),
                    value: &release_criteria_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "batchGroupName".into(),
                },
                register_interface::ResultField {
                    name: "integrationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "releaseCriteria".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationAccountBatchConfigurationResult {
            batch_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchGroupName").unwrap(),
            ),
            integration_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationAccountName").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            release_criteria: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseCriteria").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
