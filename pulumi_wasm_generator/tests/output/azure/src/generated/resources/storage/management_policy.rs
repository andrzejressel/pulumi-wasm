/// Manages an Azure Storage Account Management Policy.
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
///             .name("resourceGroupName")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_kind("BlobStorage")
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("storageaccountname")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleManagementPolicy = management_policy::create(
///         "exampleManagementPolicy",
///         ManagementPolicyArgs::builder()
///             .rules(
///                 vec![
///                     ManagementPolicyRule::builder()
///                     .actions(ManagementPolicyRuleActions::builder()
///                     .baseBlob(ManagementPolicyRuleActionsBaseBlob::builder()
///                     .deleteAfterDaysSinceModificationGreaterThan(100)
///                     .tierToArchiveAfterDaysSinceModificationGreaterThan(50)
///                     .tierToCoolAfterDaysSinceModificationGreaterThan(10).build_struct())
///                     .snapshot(ManagementPolicyRuleActionsSnapshot::builder()
///                     .deleteAfterDaysSinceCreationGreaterThan(30).build_struct())
///                     .build_struct()).enabled(true)
///                     .filters(ManagementPolicyRuleFilters::builder()
///                     .blobTypes(vec!["blockBlob",])
///                     .matchBlobIndexTags(vec![ManagementPolicyRuleFiltersMatchBlobIndexTag::builder()
///                     .name("tag1").operation("==").value("val1").build_struct(),])
///                     .prefixMatches(vec!["container1/prefix1",]).build_struct())
///                     .name("rule1").build_struct(), ManagementPolicyRule::builder()
///                     .actions(ManagementPolicyRuleActions::builder()
///                     .baseBlob(ManagementPolicyRuleActionsBaseBlob::builder()
///                     .deleteAfterDaysSinceModificationGreaterThan(101)
///                     .tierToArchiveAfterDaysSinceModificationGreaterThan(51)
///                     .tierToCoolAfterDaysSinceModificationGreaterThan(11).build_struct())
///                     .snapshot(ManagementPolicyRuleActionsSnapshot::builder()
///                     .changeTierToArchiveAfterDaysSinceCreation(90)
///                     .changeTierToCoolAfterDaysSinceCreation(23)
///                     .deleteAfterDaysSinceCreationGreaterThan(31).build_struct())
///                     .version(ManagementPolicyRuleActionsVersion::builder()
///                     .changeTierToArchiveAfterDaysSinceCreation(9)
///                     .changeTierToCoolAfterDaysSinceCreation(90)
///                     .deleteAfterDaysSinceCreation(3).build_struct()).build_struct())
///                     .enabled(false).filters(ManagementPolicyRuleFilters::builder()
///                     .blobTypes(vec!["blockBlob",])
///                     .prefixMatches(vec!["container2/prefix1", "container2/prefix2",])
///                     .build_struct()).name("rule2").build_struct(),
///                 ],
///             )
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Account Management Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/managementPolicy:ManagementPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Storage/storageAccounts/myaccountname/managementPolicies/default
/// ```
///
pub mod management_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementPolicyArgs {
        /// A `rule` block as documented below.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::ManagementPolicyRule>>,
        >,
        /// Specifies the id of the storage account to apply the management policy to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagementPolicyResult {
        /// A `rule` block as documented below.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::ManagementPolicyRule>>,
        >,
        /// Specifies the id of the storage account to apply the management policy to. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ManagementPolicyArgs) -> ManagementPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/managementPolicy:ManagementPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagementPolicyResult {
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
        }
    }
}