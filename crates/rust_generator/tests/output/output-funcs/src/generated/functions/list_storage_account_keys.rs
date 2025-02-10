#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod list_storage_account_keys {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListStorageAccountKeysArgs {
        /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies type of the key to be listed. Possible value is kerb.
        #[builder(into, default)]
        pub expand: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group within the user's subscription. The name is case insensitive.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ListStorageAccountKeysResult {
        /// Gets the list of storage account keys and their properties for the specified storage account.
        pub keys: pulumi_gestalt_rust::Output<
            Vec<super::super::types::StorageAccountKeyResponse>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: ListStorageAccountKeysArgs,
    ) -> ListStorageAccountKeysResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let expand_binding = args.expand.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "mypkg::listStorageAccountKeys".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expand".into(),
                    value: expand_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        ListStorageAccountKeysResult {
            keys: o.get_field("keys"),
        }
    }
}
