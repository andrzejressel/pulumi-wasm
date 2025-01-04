/// Manages a Storage Object Replication.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dst = resource_group::create(
///         "dst",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("dstResourceGroupName")
///             .build_struct(),
///     );
///     let dstAccount = account::create(
///         "dstAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .blob_properties(
///                 AccountBlobProperties::builder()
///                     .changeFeedEnabled(true)
///                     .versioningEnabled(true)
///                     .build_struct(),
///             )
///             .location("${dst.location}")
///             .name("dststorageaccount")
///             .resource_group_name("${dst.name}")
///             .build_struct(),
///     );
///     let dstContainer = container::create(
///         "dstContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("dststrcontainer")
///             .storage_account_name("${dstAccount.name}")
///             .build_struct(),
///     );
///     let example = object_replication::create(
///         "example",
///         ObjectReplicationArgs::builder()
///             .destination_storage_account_id("${dstAccount.id}")
///             .rules(
///                 vec![
///                     ObjectReplicationRule::builder()
///                     .destinationContainerName("${dstContainer.name}")
///                     .sourceContainerName("${srcContainer.name}").build_struct(),
///                 ],
///             )
///             .source_storage_account_id("${srcAccount.id}")
///             .build_struct(),
///     );
///     let src = resource_group::create(
///         "src",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("srcResourceGroupName")
///             .build_struct(),
///     );
///     let srcAccount = account::create(
///         "srcAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .blob_properties(
///                 AccountBlobProperties::builder()
///                     .changeFeedEnabled(true)
///                     .versioningEnabled(true)
///                     .build_struct(),
///             )
///             .location("${src.location}")
///             .name("srcstorageaccount")
///             .resource_group_name("${src.name}")
///             .build_struct(),
///     );
///     let srcContainer = container::create(
///         "srcContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("srcstrcontainer")
///             .storage_account_name("${srcAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Object Replication Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/objectReplication:ObjectReplication example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/storageAccount1/objectReplicationPolicies/objectReplicationPolicy1;/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group2/providers/Microsoft.Storage/storageAccounts/storageAccount2/objectReplicationPolicies/objectReplicationPolicy2
/// ```
///
pub mod object_replication {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectReplicationArgs {
        /// The ID of the destination storage account. Changing this forces a new Storage Object Replication to be created.
        #[builder(into)]
        pub destination_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// One or more `rules` blocks as defined below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::ObjectReplicationRule>,
        >,
        /// The ID of the source storage account. Changing this forces a new Storage Object Replication to be created.
        #[builder(into)]
        pub source_storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ObjectReplicationResult {
        /// The ID of the Object Replication in the destination storage account.
        pub destination_object_replication_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the destination storage account. Changing this forces a new Storage Object Replication to be created.
        pub destination_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// One or more `rules` blocks as defined below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::ObjectReplicationRule>,
        >,
        /// The ID of the Object Replication in the source storage account.
        pub source_object_replication_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the source storage account. Changing this forces a new Storage Object Replication to be created.
        pub source_storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ObjectReplicationArgs) -> ObjectReplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_storage_account_id_binding = args
            .destination_storage_account_id
            .get_inner();
        let rules_binding = args.rules.get_inner();
        let source_storage_account_id_binding = args
            .source_storage_account_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/objectReplication:ObjectReplication".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationStorageAccountId".into(),
                    value: &destination_storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "sourceStorageAccountId".into(),
                    value: &source_storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationObjectReplicationId".into(),
                },
                register_interface::ResultField {
                    name: "destinationStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "sourceObjectReplicationId".into(),
                },
                register_interface::ResultField {
                    name: "sourceStorageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ObjectReplicationResult {
            destination_object_replication_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationObjectReplicationId").unwrap(),
            ),
            destination_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationStorageAccountId").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            source_object_replication_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceObjectReplicationId").unwrap(),
            ),
            source_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceStorageAccountId").unwrap(),
            ),
        }
    }
}
