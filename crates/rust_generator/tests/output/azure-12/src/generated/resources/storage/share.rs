/// Manages a File Share within Azure Storage.
///
/// > **Note** The storage share supports two storage tiers: premium and standard. Standard file shares are created in general purpose (GPv1 or GPv2) storage accounts and premium file shares are created in FileStorage storage accounts. For further information, refer to the section "What storage tiers are supported in Azure Files?" of [documentation](https://docs.microsoft.com/azure/storage/files/storage-files-faq#general).
///
/// > **Note on Authentication** Shared Key authentication will always be used for this resource, as AzureAD authentication is not supported by the Storage API for files.
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
///             .name("azuretest")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("azureteststorage")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleShare = share::create(
///         "exampleShare",
///         ShareArgs::builder()
///             .acls(
///                 vec![
///                     ShareAcl::builder()
///                     .accessPolicies(vec![ShareAclAccessPolicy::builder()
///                     .expiry("2019-07-02T10:38:21Z").permissions("rwdl")
///                     .start("2019-07-02T09:38:21Z").build_struct(),])
///                     .id("MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI").build_struct(),
///                 ],
///             )
///             .name("sharename")
///             .quota(50)
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Shares can be imported using the `id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/share:Share exampleShare /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Storage/storageAccounts/myAccount/fileServices/default/shares/exampleShare
/// ```
///
pub mod share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareArgs {
        /// The access tier of the File Share. Possible values are `Hot`, `Cool` and `TransactionOptimized`, `Premium`.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` requires `Premium` `access_tier`.
        #[builder(into, default)]
        pub access_tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `acl` blocks as defined below.
        #[builder(into, default)]
        pub acls: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::ShareAcl>>,
        >,
        /// The protocol used for the share. Possible values are `SMB` and `NFS`. The `SMB` indicates the share can be accessed by SMBv3.0, SMBv2.1 and REST. The `NFS` indicates the share can be accessed by NFSv4.1. Defaults to `SMB`. Changing this forces a new resource to be created.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` is required for the `NFS` protocol.
        #[builder(into, default)]
        pub enabled_protocol: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of MetaData for this File Share.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the share. Must be unique within the storage account where the share is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The maximum size of the share, in gigabytes.
        ///
        /// ~>**NOTE:** For Standard storage accounts, by default this must be `1` GB (or higher) and at most `5120` GB (`5` TB). This can be set to a value larger than `5120` GB if `large_file_share_enabled` is set to `true` in the parent `azure.storage.Account`.
        ///
        /// ~>**NOTE:** For Premium FileStorage storage accounts, this must be greater than `100` GB and at most `102400` GB (`100` TB).
        #[builder(into)]
        pub quota: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ShareResult {
        /// The access tier of the File Share. Possible values are `Hot`, `Cool` and `TransactionOptimized`, `Premium`.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` requires `Premium` `access_tier`.
        pub access_tier: pulumi_wasm_rust::Output<String>,
        /// One or more `acl` blocks as defined below.
        pub acls: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::storage::ShareAcl>>,
        >,
        /// The protocol used for the share. Possible values are `SMB` and `NFS`. The `SMB` indicates the share can be accessed by SMBv3.0, SMBv2.1 and REST. The `NFS` indicates the share can be accessed by NFSv4.1. Defaults to `SMB`. Changing this forces a new resource to be created.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` is required for the `NFS` protocol.
        pub enabled_protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of MetaData for this File Share.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the share. Must be unique within the storage account where the share is located. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The maximum size of the share, in gigabytes.
        ///
        /// ~>**NOTE:** For Standard storage accounts, by default this must be `1` GB (or higher) and at most `5120` GB (`5` TB). This can be set to a value larger than `5120` GB if `large_file_share_enabled` is set to `true` in the parent `azure.storage.Account`.
        ///
        /// ~>**NOTE:** For Premium FileStorage storage accounts, this must be greater than `100` GB and at most `102400` GB (`100` TB).
        pub quota: pulumi_wasm_rust::Output<i32>,
        /// The Resource Manager ID of this File Share.
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of the File Share
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ShareArgs,
    ) -> ShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_tier_binding = args.access_tier.get_output(context).get_inner();
        let acls_binding = args.acls.get_output(context).get_inner();
        let enabled_protocol_binding = args
            .enabled_protocol
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let quota_binding = args.quota.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/share:Share".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessTier".into(),
                    value: &access_tier_binding,
                },
                register_interface::ObjectField {
                    name: "acls".into(),
                    value: &acls_binding,
                },
                register_interface::ObjectField {
                    name: "enabledProtocol".into(),
                    value: &enabled_protocol_binding,
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
                    name: "quota".into(),
                    value: &quota_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ShareResult {
            access_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessTier"),
            ),
            acls: pulumi_wasm_rust::__private::into_domain(o.extract_field("acls")),
            enabled_protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabledProtocol"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            quota: pulumi_wasm_rust::__private::into_domain(o.extract_field("quota")),
            resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceManagerId"),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
            url: pulumi_wasm_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
