pub mod get_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// The name of the Batch account.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where this Batch account exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// The account endpoint used to interact with the Batch service.
        pub account_endpoint: pulumi_wasm_rust::Output<String>,
        /// The `encryption` block that describes the Azure KeyVault key reference used to encrypt data for the Azure Batch account.
        pub encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetAccountEncryption>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The `key_vault_reference` block that describes the Azure KeyVault reference to use when deploying the Azure Batch account using the `UserSubscription` pool allocation mode.
        pub key_vault_references: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetAccountKeyVaultReference>,
        >,
        /// The Azure Region in which this Batch account exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Batch account name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The pool allocation mode configured for this Batch account.
        pub pool_allocation_mode: pulumi_wasm_rust::Output<String>,
        /// The Batch account primary access key.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Batch account secondary access key.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account used for this Batch account.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the Batch account.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccountArgs) -> GetAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:batch/getAccount:getAccount".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "encryptions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultReferences".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "poolAllocationMode".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountResult {
            account_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountEndpoint").unwrap(),
            ),
            encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultReferences").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pool_allocation_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolAllocationMode").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
