pub mod get_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// The name of the Batch account.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where this Batch account exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAccountArgs,
    ) -> GetAccountResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccountResult {
            account_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountEndpoint"),
            ),
            encryptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptions"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_vault_references: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultReferences"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            pool_allocation_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("poolAllocationMode"),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
