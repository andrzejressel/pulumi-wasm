/// Manages a Logic App Integration Account Batch Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account_batch_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountBatchConfigurationArgs {
        /// The batch group name of the Logic App Integration Batch Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub batch_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Batch Configuration.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Batch Configuration. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `release_criteria` block as documented below, which is used to select the criteria to meet before processing each batch.
        #[builder(into)]
        pub release_criteria: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteria,
        >,
        /// The name of the Resource Group where the Logic App Integration Account Batch Configuration should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountBatchConfigurationResult {
        /// The batch group name of the Logic App Integration Batch Configuration. Changing this forces a new resource to be created.
        pub batch_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Batch Configuration.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Batch Configuration. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `release_criteria` block as documented below, which is used to select the criteria to meet before processing each batch.
        pub release_criteria: pulumi_gestalt_rust::Output<
            super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteria,
        >,
        /// The name of the Resource Group where the Logic App Integration Account Batch Configuration should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountBatchConfigurationArgs,
    ) -> IntegrationAccountBatchConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let batch_group_name_binding = args.batch_group_name.get_output(context);
        let integration_account_name_binding = args
            .integration_account_name
            .get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let release_criteria_binding = args.release_criteria.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountBatchConfiguration:IntegrationAccountBatchConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchGroupName".into(),
                    value: batch_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationAccountName".into(),
                    value: integration_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseCriteria".into(),
                    value: release_criteria_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationAccountBatchConfigurationResult {
            batch_group_name: o.get_field("batchGroupName"),
            integration_account_name: o.get_field("integrationAccountName"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            release_criteria: o.get_field("releaseCriteria"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
