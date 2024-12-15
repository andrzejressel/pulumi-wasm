//! The response from the ListKeys operation.
//! API Version: 2021-02-01.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListStorageAccountKeysArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    #[builder(into)]
    pub account_name: pulumi_wasm_rust::Output<String>,
    /// Specifies type of the key to be listed. Possible value is kerb.
    #[builder(into, default)]
    pub expand: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    #[builder(into)]
    pub resource_group_name: pulumi_wasm_rust::Output<String>,
}

pub struct ListStorageAccountKeysResult {
    /// Gets the list of storage account keys and their properties for the specified storage account.
    pub keys: pulumi_wasm_rust::Output<Vec<crate::types::StorageAccountKeyResponse>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: ListStorageAccountKeysArgs
) -> ListStorageAccountKeysResult {

    let result = crate::bindings::pulumi::mypkg::list_storage_account_keys::invoke(
        &crate::bindings::pulumi::mypkg::list_storage_account_keys::Args {
                account_name: &args.account_name.get_inner(),
                expand: &args.expand.get_inner(),
                resource_group_name: &args.resource_group_name.get_inner(),
        }
    );

    ListStorageAccountKeysResult {
        keys: crate::into_domain(result.keys),
    }
}
