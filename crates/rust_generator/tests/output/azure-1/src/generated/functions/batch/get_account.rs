#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// The name of the Batch account.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where this Batch account exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// The account endpoint used to interact with the Batch service.
        pub account_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The `encryption` block that describes the Azure KeyVault key reference used to encrypt data for the Azure Batch account.
        pub encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetAccountEncryption>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The `key_vault_reference` block that describes the Azure KeyVault reference to use when deploying the Azure Batch account using the `UserSubscription` pool allocation mode.
        pub key_vault_references: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::batch::GetAccountKeyVaultReference>,
        >,
        /// The Azure Region in which this Batch account exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Batch account name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The pool allocation mode configured for this Batch account.
        pub pool_allocation_mode: pulumi_gestalt_rust::Output<String>,
        /// The Batch account primary access key.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Batch account secondary access key.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account used for this Batch account.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the Batch account.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountArgs,
    ) -> GetAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:batch/getAccount:getAccount".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountResult {
            account_endpoint: o.get_field("accountEndpoint"),
            encryptions: o.get_field("encryptions"),
            id: o.get_field("id"),
            key_vault_references: o.get_field("keyVaultReferences"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pool_allocation_mode: o.get_field("poolAllocationMode"),
            primary_access_key: o.get_field("primaryAccessKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            storage_account_id: o.get_field("storageAccountId"),
            tags: o.get_field("tags"),
        }
    }
}
