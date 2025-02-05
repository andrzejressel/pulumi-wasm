/// Manages a Storage Blob Inventory Policy.
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .blob_properties(
///                 AccountBlobProperties::builder().versioningEnabled(true).build_struct(),
///             )
///             .location("${example.location}")
///             .name("examplestoracc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleBlobInventoryPolicy = blob_inventory_policy::create(
///         "exampleBlobInventoryPolicy",
///         BlobInventoryPolicyArgs::builder()
///             .rules(
///                 vec![
///                     BlobInventoryPolicyRule::builder().format("Csv").name("rule1")
///                     .schedule("Daily").schemaFields(vec!["Name", "Last-Modified",])
///                     .scope("Container").storageContainerName("${exampleContainer.name}")
///                     .build_struct(),
///                 ],
///             )
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("examplecontainer")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Blob Inventory Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/blobInventoryPolicy:BlobInventoryPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/storageAccount1
/// ```
///
pub mod blob_inventory_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlobInventoryPolicyArgs {
        /// One or more `rules` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::storage::BlobInventoryPolicyRule>,
        >,
        /// The ID of the storage account to apply this Blob Inventory Policy to. Changing this forces a new Storage Blob Inventory Policy to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BlobInventoryPolicyResult {
        /// One or more `rules` blocks as defined below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::BlobInventoryPolicyRule>,
        >,
        /// The ID of the storage account to apply this Blob Inventory Policy to. Changing this forces a new Storage Blob Inventory Policy to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BlobInventoryPolicyArgs,
    ) -> BlobInventoryPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/blobInventoryPolicy:BlobInventoryPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        BlobInventoryPolicyResult {
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
        }
    }
}
