/// Manages a File Share within Azure Storage.
///
/// > **Note** The storage share supports two storage tiers: premium and standard. Standard file shares are created in general purpose (GPv1 or GPv2) storage accounts and premium file shares are created in FileStorage storage accounts. For further information, refer to the section "What storage tiers are supported in Azure Files?" of [documentation](https://docs.microsoft.com/azure/storage/files/storage-files-faq#general).
///
/// > **Note on Authentication** Shared Key authentication will always be used for this resource, as AzureAD authentication is not supported by the Storage API for files.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareArgs {
        /// The access tier of the File Share. Possible values are `Hot`, `Cool` and `TransactionOptimized`, `Premium`.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` requires `Premium` `access_tier`.
        #[builder(into, default)]
        pub access_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `acl` blocks as defined below.
        #[builder(into, default)]
        pub acls: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::ShareAcl>>,
        >,
        /// The protocol used for the share. Possible values are `SMB` and `NFS`. The `SMB` indicates the share can be accessed by SMBv3.0, SMBv2.1 and REST. The `NFS` indicates the share can be accessed by NFSv4.1. Defaults to `SMB`. Changing this forces a new resource to be created.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` is required for the `NFS` protocol.
        #[builder(into, default)]
        pub enabled_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of MetaData for this File Share.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the share. Must be unique within the storage account where the share is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum size of the share, in gigabytes.
        ///
        /// ~>**NOTE:** For Standard storage accounts, by default this must be `1` GB (or higher) and at most `5120` GB (`5` TB). This can be set to a value larger than `5120` GB if `large_file_share_enabled` is set to `true` in the parent `azure.storage.Account`.
        ///
        /// ~>**NOTE:** For Premium FileStorage storage accounts, this must be greater than `100` GB and at most `102400` GB (`100` TB).
        #[builder(into)]
        pub quota: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ShareResult {
        /// The access tier of the File Share. Possible values are `Hot`, `Cool` and `TransactionOptimized`, `Premium`.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` requires `Premium` `access_tier`.
        pub access_tier: pulumi_gestalt_rust::Output<String>,
        /// One or more `acl` blocks as defined below.
        pub acls: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::storage::ShareAcl>>,
        >,
        /// The protocol used for the share. Possible values are `SMB` and `NFS`. The `SMB` indicates the share can be accessed by SMBv3.0, SMBv2.1 and REST. The `NFS` indicates the share can be accessed by NFSv4.1. Defaults to `SMB`. Changing this forces a new resource to be created.
        ///
        /// ~>**NOTE:** The `FileStorage` `account_kind` of the `azure.storage.Account` is required for the `NFS` protocol.
        pub enabled_protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of MetaData for this File Share.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the share. Must be unique within the storage account where the share is located. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The maximum size of the share, in gigabytes.
        ///
        /// ~>**NOTE:** For Standard storage accounts, by default this must be `1` GB (or higher) and at most `5120` GB (`5` TB). This can be set to a value larger than `5120` GB if `large_file_share_enabled` is set to `true` in the parent `azure.storage.Account`.
        ///
        /// ~>**NOTE:** For Premium FileStorage storage accounts, this must be greater than `100` GB and at most `102400` GB (`100` TB).
        pub quota: pulumi_gestalt_rust::Output<i32>,
        /// The Resource Manager ID of this File Share.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the storage account in which to create the share. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        pub storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the File Share
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ShareArgs,
    ) -> ShareResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_tier_binding = args.access_tier.get_output(context);
        let acls_binding = args.acls.get_output(context);
        let enabled_protocol_binding = args.enabled_protocol.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let quota_binding = args.quota.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/share:Share".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessTier".into(),
                    value: &access_tier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acls".into(),
                    value: &acls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledProtocol".into(),
                    value: &enabled_protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quota".into(),
                    value: &quota_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ShareResult {
            access_tier: o.get_field("accessTier"),
            acls: o.get_field("acls"),
            enabled_protocol: o.get_field("enabledProtocol"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            quota: o.get_field("quota"),
            resource_manager_id: o.get_field("resourceManagerId"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_name: o.get_field("storageAccountName"),
            url: o.get_field("url"),
        }
    }
}
