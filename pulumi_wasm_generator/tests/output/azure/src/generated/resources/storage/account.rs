/// Manages an Azure Storage Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///       tags:
///         environment: staging
/// ```
///
///
/// ### With Network Rules
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: virtnetname
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: subnetname
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       serviceEndpoints:
///         - Microsoft.Sql
///         - Microsoft.Storage
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       networkRules:
///         defaultAction: Deny
///         ipRules:
///           - 100.0.0.1
///         virtualNetworkSubnetIds:
///           - ${exampleSubnet.id}
///       tags:
///         environment: staging
/// ```
///
/// ## Import
///
/// Storage Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/account:Account storageAcc1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Storage/storageAccounts/myaccount
/// ```
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// Defines the access tier for `BlobStorage`, `FileStorage` and `StorageV2` accounts. Valid options are `Hot` and `Cool`, defaults to `Hot`.
        #[builder(into, default)]
        pub access_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the Kind of account. Valid options are `BlobStorage`, `BlockBlobStorage`, `FileStorage`, `Storage` and `StorageV2`. Defaults to `StorageV2`.
        ///
        /// > **Note:** Changing the `account_kind` value from `Storage` to `StorageV2` will not trigger a force new on the storage account, it will only upgrade the existing storage account from `Storage` to `StorageV2` keeping the existing storage account in place.
        #[builder(into, default)]
        pub account_kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the type of replication to use for this storage account. Valid options are `LRS`, `GRS`, `RAGRS`, `ZRS`, `GZRS` and `RAGZRS`. Changing this forces a new resource to be created when types `LRS`, `GRS` and `RAGRS` are changed to `ZRS`, `GZRS` or `RAGZRS` and vice versa.
        #[builder(into)]
        pub account_replication_type: pulumi_wasm_rust::Output<String>,
        /// Defines the Tier to use for this storage account. Valid options are `Standard` and `Premium`. For `BlockBlobStorage` and `FileStorage` accounts only `Premium` is valid. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Blobs with a tier of `Premium` are of account kind `StorageV2`.
        #[builder(into)]
        pub account_tier: pulumi_wasm_rust::Output<String>,
        /// Allow or disallow nested items within this Account to opt into being public. Defaults to `true`.
        ///
        /// > **Note:** At this time `allow_nested_items_to_be_public` is only supported in the Public Cloud, China Cloud, and US Government Cloud.
        #[builder(into, default)]
        pub allow_nested_items_to_be_public: pulumi_wasm_rust::Output<Option<bool>>,
        /// Restrict copy to and from Storage Accounts within an AAD tenant or with Private Links to the same VNet. Possible values are `AAD` and `PrivateLink`.
        #[builder(into, default)]
        pub allowed_copy_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// A `azure_files_authentication` block as defined below.
        #[builder(into, default)]
        pub azure_files_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountAzureFilesAuthentication>,
        >,
        /// A `blob_properties` block as defined below.
        #[builder(into, default)]
        pub blob_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountBlobProperties>,
        >,
        /// Should cross Tenant replication be enabled? Defaults to `false`.
        #[builder(into, default)]
        pub cross_tenant_replication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `custom_domain` block as documented below.
        #[builder(into, default)]
        pub custom_domain: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountCustomDomain>,
        >,
        /// A `customer_managed_key` block as documented below.
        ///
        /// > **Note:** It's possible to define a Customer Managed Key both within either the `customer_managed_key` block or by using the `azure.storage.CustomerManagedKey` resource. However, it's not possible to use both methods to manage a Customer Managed Key for a Storage Account, since these will conflict. When using the `azure.storage.CustomerManagedKey` resource, you will need to use `ignore_changes` on the `customer_managed_key` block.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountCustomerManagedKey>,
        >,
        /// Default to Azure Active Directory authorization in the Azure portal when accessing the Storage Account. The default value is `false`
        #[builder(into, default)]
        pub default_to_oauth_authentication: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies which DNS endpoint type to use. Possible values are `Standard` and `AzureDnsZone`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Azure DNS zone support requires `PartitionedDns` feature to be enabled. To enable this feature for your subscription, use the following command: `az feature register --namespace "Microsoft.Storage" --name "PartitionedDns"`.
        #[builder(into, default)]
        pub dns_endpoint_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Storage Account should exist. Changing this forces a new Storage Account to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which forces HTTPS if enabled, see [here](https://docs.microsoft.com/azure/storage/storage-require-secure-transfer/) for more information. Defaults to `true`.
        #[builder(into, default)]
        pub https_traffic_only_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountIdentity>,
        >,
        /// An `immutability_policy` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub immutability_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountImmutabilityPolicy>,
        >,
        /// Is infrastructure encryption enabled? Changing this forces a new resource to be created. Defaults to `false`.
        ///
        /// > **Note:** This can only be `true` when `account_kind` is `StorageV2` or when `account_tier` is `Premium` *and* `account_kind` is one of `BlockBlobStorage` or `FileStorage`.
        #[builder(into, default)]
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Hierarchical Namespace enabled? This can be used with Azure Data Lake Storage Gen 2 ([see here for more information](https://docs.microsoft.com/azure/storage/blobs/data-lake-storage-quickstart-create-account/)). Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be `true` when `account_tier` is `Standard` or when `account_tier` is `Premium` *and* `account_kind` is `BlockBlobStorage`
        #[builder(into, default)]
        pub is_hns_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Are Large File Shares Enabled? Defaults to `false`.
        ///
        /// > **Note:** Large File Shares are enabled by default when using an `account_kind` of `FileStorage`.
        #[builder(into, default)]
        pub large_file_share_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Local User Enabled? Defaults to `true`.
        #[builder(into, default)]
        pub local_user_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum supported TLS version for the storage account. Possible values are `TLS1_0`, `TLS1_1`, and `TLS1_2`. Defaults to `TLS1_2` for new storage accounts.
        ///
        /// > **Note:** At this time `min_tls_version` is only supported in the Public Cloud, China Cloud, and US Government Cloud.
        #[builder(into, default)]
        pub min_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the storage account. Only lowercase Alphanumeric characters allowed. Changing this forces a new resource to be created. This must be unique across the entire Azure service, not just within the resource group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_rules` block as documented below.
        #[builder(into, default)]
        pub network_rules: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountNetworkRules>,
        >,
        /// Is NFSv3 protocol enabled? Changing this forces a new resource to be created. Defaults to `false`.
        ///
        /// > **Note:** This can only be `true` when `account_tier` is `Standard` and `account_kind` is `StorageV2`, or `account_tier` is `Premium` and `account_kind` is `BlockBlobStorage`. Additionally, the `is_hns_enabled` is `true` and `account_replication_type` must be `LRS` or `RAGRS`.
        #[builder(into, default)]
        pub nfsv3_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the public network access is enabled? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The encryption type of the queue service. Possible values are `Service` and `Account`. Changing this forces a new resource to be created. Default value is `Service`.
        #[builder(into, default)]
        pub queue_encryption_key_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A `queue_properties` block as defined below.
        ///
        /// > **Note:** `queue_properties` can only be configured when `account_tier` is set to `Standard` and `account_kind` is set to either `Storage` or `StorageV2`.
        #[builder(into, default)]
        pub queue_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountQueueProperties>,
        >,
        /// The name of the resource group in which to create the storage account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `routing` block as defined below.
        #[builder(into, default)]
        pub routing: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountRouting>,
        >,
        /// A `sas_policy` block as defined below.
        #[builder(into, default)]
        pub sas_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountSasPolicy>,
        >,
        /// Boolean, enable SFTP for the storage account
        ///
        /// > **Note:** SFTP support requires `is_hns_enabled` set to `true`. [More information on SFTP support can be found here](https://learn.microsoft.com/azure/storage/blobs/secure-file-transfer-protocol-support). Defaults to `false`
        #[builder(into, default)]
        pub sftp_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `share_properties` block as defined below.
        ///
        /// > **Note:** `share_properties` can only be configured when either `account_tier` is `Standard` and `account_kind` is either `Storage` or `StorageV2` - or when `account_tier` is `Premium` and `account_kind` is `FileStorage`.
        #[builder(into, default)]
        pub share_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountShareProperties>,
        >,
        #[builder(into, default)]
        pub shared_access_key_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `static_website` block as defined below.
        ///
        /// > **Note:** `static_website` can only be set when the `account_kind` is set to `StorageV2` or `BlockBlobStorage`.
        ///
        /// > **Note:** If `static_website` is specified, the service will automatically create a `azure.storage.Container` named `$web`.
        #[builder(into, default)]
        pub static_website: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountStaticWebsite>,
        >,
        /// The encryption type of the table service. Possible values are `Service` and `Account`. Changing this forces a new resource to be created. Default value is `Service`.
        ///
        /// > **Note:** `queue_encryption_key_type` and `table_encryption_key_type` cannot be set to `Account` when `account_kind` is set `Storage`
        #[builder(into, default)]
        pub table_encryption_key_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// Defines the access tier for `BlobStorage`, `FileStorage` and `StorageV2` accounts. Valid options are `Hot` and `Cool`, defaults to `Hot`.
        pub access_tier: pulumi_wasm_rust::Output<String>,
        /// Defines the Kind of account. Valid options are `BlobStorage`, `BlockBlobStorage`, `FileStorage`, `Storage` and `StorageV2`. Defaults to `StorageV2`.
        ///
        /// > **Note:** Changing the `account_kind` value from `Storage` to `StorageV2` will not trigger a force new on the storage account, it will only upgrade the existing storage account from `Storage` to `StorageV2` keeping the existing storage account in place.
        pub account_kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the type of replication to use for this storage account. Valid options are `LRS`, `GRS`, `RAGRS`, `ZRS`, `GZRS` and `RAGZRS`. Changing this forces a new resource to be created when types `LRS`, `GRS` and `RAGRS` are changed to `ZRS`, `GZRS` or `RAGZRS` and vice versa.
        pub account_replication_type: pulumi_wasm_rust::Output<String>,
        /// Defines the Tier to use for this storage account. Valid options are `Standard` and `Premium`. For `BlockBlobStorage` and `FileStorage` accounts only `Premium` is valid. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Blobs with a tier of `Premium` are of account kind `StorageV2`.
        pub account_tier: pulumi_wasm_rust::Output<String>,
        /// Allow or disallow nested items within this Account to opt into being public. Defaults to `true`.
        ///
        /// > **Note:** At this time `allow_nested_items_to_be_public` is only supported in the Public Cloud, China Cloud, and US Government Cloud.
        pub allow_nested_items_to_be_public: pulumi_wasm_rust::Output<Option<bool>>,
        /// Restrict copy to and from Storage Accounts within an AAD tenant or with Private Links to the same VNet. Possible values are `AAD` and `PrivateLink`.
        pub allowed_copy_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// A `azure_files_authentication` block as defined below.
        pub azure_files_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountAzureFilesAuthentication>,
        >,
        /// A `blob_properties` block as defined below.
        pub blob_properties: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountBlobProperties,
        >,
        /// Should cross Tenant replication be enabled? Defaults to `false`.
        pub cross_tenant_replication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `custom_domain` block as documented below.
        pub custom_domain: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountCustomDomain>,
        >,
        /// A `customer_managed_key` block as documented below.
        ///
        /// > **Note:** It's possible to define a Customer Managed Key both within either the `customer_managed_key` block or by using the `azure.storage.CustomerManagedKey` resource. However, it's not possible to use both methods to manage a Customer Managed Key for a Storage Account, since these will conflict. When using the `azure.storage.CustomerManagedKey` resource, you will need to use `ignore_changes` on the `customer_managed_key` block.
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountCustomerManagedKey>,
        >,
        /// Default to Azure Active Directory authorization in the Azure portal when accessing the Storage Account. The default value is `false`
        pub default_to_oauth_authentication: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies which DNS endpoint type to use. Possible values are `Standard` and `AzureDnsZone`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Azure DNS zone support requires `PartitionedDns` feature to be enabled. To enable this feature for your subscription, use the following command: `az feature register --namespace "Microsoft.Storage" --name "PartitionedDns"`.
        pub dns_endpoint_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Storage Account should exist. Changing this forces a new Storage Account to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which forces HTTPS if enabled, see [here](https://docs.microsoft.com/azure/storage/storage-require-secure-transfer/) for more information. Defaults to `true`.
        pub https_traffic_only_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountIdentity>,
        >,
        /// An `immutability_policy` block as defined below. Changing this forces a new resource to be created.
        pub immutability_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountImmutabilityPolicy>,
        >,
        /// Is infrastructure encryption enabled? Changing this forces a new resource to be created. Defaults to `false`.
        ///
        /// > **Note:** This can only be `true` when `account_kind` is `StorageV2` or when `account_tier` is `Premium` *and* `account_kind` is one of `BlockBlobStorage` or `FileStorage`.
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Hierarchical Namespace enabled? This can be used with Azure Data Lake Storage Gen 2 ([see here for more information](https://docs.microsoft.com/azure/storage/blobs/data-lake-storage-quickstart-create-account/)). Changing this forces a new resource to be created.
        ///
        /// > **Note:** This can only be `true` when `account_tier` is `Standard` or when `account_tier` is `Premium` *and* `account_kind` is `BlockBlobStorage`
        pub is_hns_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Are Large File Shares Enabled? Defaults to `false`.
        ///
        /// > **Note:** Large File Shares are enabled by default when using an `account_kind` of `FileStorage`.
        pub large_file_share_enabled: pulumi_wasm_rust::Output<bool>,
        /// Is Local User Enabled? Defaults to `true`.
        pub local_user_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum supported TLS version for the storage account. Possible values are `TLS1_0`, `TLS1_1`, and `TLS1_2`. Defaults to `TLS1_2` for new storage accounts.
        ///
        /// > **Note:** At this time `min_tls_version` is only supported in the Public Cloud, China Cloud, and US Government Cloud.
        pub min_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the storage account. Only lowercase Alphanumeric characters allowed. Changing this forces a new resource to be created. This must be unique across the entire Azure service, not just within the resource group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_rules` block as documented below.
        pub network_rules: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountNetworkRules,
        >,
        /// Is NFSv3 protocol enabled? Changing this forces a new resource to be created. Defaults to `false`.
        ///
        /// > **Note:** This can only be `true` when `account_tier` is `Standard` and `account_kind` is `StorageV2`, or `account_tier` is `Premium` and `account_kind` is `BlockBlobStorage`. Additionally, the `is_hns_enabled` is `true` and `account_replication_type` must be `LRS` or `RAGRS`.
        pub nfsv3_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The primary access key for the storage account.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The connection string associated with the primary blob location.
        pub primary_blob_connection_string: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for blob storage in the primary location.
        pub primary_blob_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for blob storage in the primary location.
        pub primary_blob_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for blob storage in the primary location.
        pub primary_blob_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for blob storage in the primary location.
        pub primary_blob_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for blob storage in the primary location.
        pub primary_blob_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for blob storage in the primary location.
        pub primary_blob_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The connection string associated with the primary location.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for DFS storage in the primary location.
        pub primary_dfs_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for DFS storage in the primary location.
        pub primary_dfs_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for DFS storage in the primary location.
        pub primary_dfs_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for DFS storage in the primary location.
        pub primary_dfs_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for DFS storage in the primary location.
        pub primary_dfs_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for DFS storage in the primary location.
        pub primary_dfs_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for file storage in the primary location.
        pub primary_file_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for file storage in the primary location.
        pub primary_file_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for file storage in the primary location.
        pub primary_file_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for file storage in the primary location.
        pub primary_file_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for file storage in the primary location.
        pub primary_file_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for file storage in the primary location.
        pub primary_file_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The primary location of the storage account.
        pub primary_location: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for queue storage in the primary location.
        pub primary_queue_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for queue storage in the primary location.
        pub primary_queue_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for queue storage in the primary location.
        pub primary_queue_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for queue storage in the primary location.
        pub primary_queue_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for table storage in the primary location.
        pub primary_table_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for table storage in the primary location.
        pub primary_table_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for table storage in the primary location.
        pub primary_table_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for table storage in the primary location.
        pub primary_table_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for web storage in the primary location.
        pub primary_web_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for web storage in the primary location.
        pub primary_web_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for web storage in the primary location.
        pub primary_web_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for web storage in the primary location.
        pub primary_web_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for web storage in the primary location.
        pub primary_web_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for web storage in the primary location.
        pub primary_web_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// Whether the public network access is enabled? Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The encryption type of the queue service. Possible values are `Service` and `Account`. Changing this forces a new resource to be created. Default value is `Service`.
        pub queue_encryption_key_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A `queue_properties` block as defined below.
        ///
        /// > **Note:** `queue_properties` can only be configured when `account_tier` is set to `Standard` and `account_kind` is set to either `Storage` or `StorageV2`.
        pub queue_properties: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountQueueProperties,
        >,
        /// The name of the resource group in which to create the storage account. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `routing` block as defined below.
        pub routing: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountRouting,
        >,
        /// A `sas_policy` block as defined below.
        pub sas_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::storage::AccountSasPolicy>,
        >,
        /// The secondary access key for the storage account.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The connection string associated with the secondary blob location.
        pub secondary_blob_connection_string: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for blob storage in the secondary location.
        pub secondary_blob_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for blob storage in the secondary location.
        pub secondary_blob_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for blob storage in the secondary location.
        pub secondary_blob_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for blob storage in the secondary location.
        pub secondary_blob_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for blob storage in the secondary location.
        pub secondary_blob_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for blob storage in the secondary location.
        pub secondary_blob_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The connection string associated with the secondary location.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for DFS storage in the secondary location.
        pub secondary_dfs_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for DFS storage in the secondary location.
        pub secondary_dfs_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for DFS storage in the secondary location.
        pub secondary_dfs_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for DFS storage in the secondary location.
        pub secondary_dfs_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for DFS storage in the secondary location.
        pub secondary_dfs_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for DFS storage in the secondary location.
        pub secondary_dfs_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for file storage in the secondary location.
        pub secondary_file_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for file storage in the secondary location.
        pub secondary_file_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for file storage in the secondary location.
        pub secondary_file_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for file storage in the secondary location.
        pub secondary_file_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for file storage in the secondary location.
        pub secondary_file_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for file storage in the secondary location.
        pub secondary_file_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The secondary location of the storage account.
        pub secondary_location: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for queue storage in the secondary location.
        pub secondary_queue_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for queue storage in the secondary location.
        pub secondary_queue_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for queue storage in the secondary location.
        pub secondary_queue_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for queue storage in the secondary location.
        pub secondary_queue_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for table storage in the secondary location.
        pub secondary_table_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for table storage in the secondary location.
        pub secondary_table_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for table storage in the secondary location.
        pub secondary_table_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for table storage in the secondary location.
        pub secondary_table_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// The endpoint URL for web storage in the secondary location.
        pub secondary_web_endpoint: pulumi_wasm_rust::Output<String>,
        /// The hostname with port if applicable for web storage in the secondary location.
        pub secondary_web_host: pulumi_wasm_rust::Output<String>,
        /// The internet routing endpoint URL for web storage in the secondary location.
        pub secondary_web_internet_endpoint: pulumi_wasm_rust::Output<String>,
        /// The internet routing hostname with port if applicable for web storage in the secondary location.
        pub secondary_web_internet_host: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing endpoint URL for web storage in the secondary location.
        pub secondary_web_microsoft_endpoint: pulumi_wasm_rust::Output<String>,
        /// The microsoft routing hostname with port if applicable for web storage in the secondary location.
        pub secondary_web_microsoft_host: pulumi_wasm_rust::Output<String>,
        /// Boolean, enable SFTP for the storage account
        ///
        /// > **Note:** SFTP support requires `is_hns_enabled` set to `true`. [More information on SFTP support can be found here](https://learn.microsoft.com/azure/storage/blobs/secure-file-transfer-protocol-support). Defaults to `false`
        pub sftp_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `share_properties` block as defined below.
        ///
        /// > **Note:** `share_properties` can only be configured when either `account_tier` is `Standard` and `account_kind` is either `Storage` or `StorageV2` - or when `account_tier` is `Premium` and `account_kind` is `FileStorage`.
        pub share_properties: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountShareProperties,
        >,
        pub shared_access_key_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `static_website` block as defined below.
        ///
        /// > **Note:** `static_website` can only be set when the `account_kind` is set to `StorageV2` or `BlockBlobStorage`.
        ///
        /// > **Note:** If `static_website` is specified, the service will automatically create a `azure.storage.Container` named `$web`.
        pub static_website: pulumi_wasm_rust::Output<
            super::super::types::storage::AccountStaticWebsite,
        >,
        /// The encryption type of the table service. Possible values are `Service` and `Account`. Changing this forces a new resource to be created. Default value is `Service`.
        ///
        /// > **Note:** `queue_encryption_key_type` and `table_encryption_key_type` cannot be set to `Account` when `account_kind` is set `Storage`
        pub table_encryption_key_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_tier_binding = args.access_tier.get_inner();
        let account_kind_binding = args.account_kind.get_inner();
        let account_replication_type_binding = args.account_replication_type.get_inner();
        let account_tier_binding = args.account_tier.get_inner();
        let allow_nested_items_to_be_public_binding = args
            .allow_nested_items_to_be_public
            .get_inner();
        let allowed_copy_scope_binding = args.allowed_copy_scope.get_inner();
        let azure_files_authentication_binding = args
            .azure_files_authentication
            .get_inner();
        let blob_properties_binding = args.blob_properties.get_inner();
        let cross_tenant_replication_enabled_binding = args
            .cross_tenant_replication_enabled
            .get_inner();
        let custom_domain_binding = args.custom_domain.get_inner();
        let customer_managed_key_binding = args.customer_managed_key.get_inner();
        let default_to_oauth_authentication_binding = args
            .default_to_oauth_authentication
            .get_inner();
        let dns_endpoint_type_binding = args.dns_endpoint_type.get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let https_traffic_only_enabled_binding = args
            .https_traffic_only_enabled
            .get_inner();
        let identity_binding = args.identity.get_inner();
        let immutability_policy_binding = args.immutability_policy.get_inner();
        let infrastructure_encryption_enabled_binding = args
            .infrastructure_encryption_enabled
            .get_inner();
        let is_hns_enabled_binding = args.is_hns_enabled.get_inner();
        let large_file_share_enabled_binding = args.large_file_share_enabled.get_inner();
        let local_user_enabled_binding = args.local_user_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let min_tls_version_binding = args.min_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let network_rules_binding = args.network_rules.get_inner();
        let nfsv3_enabled_binding = args.nfsv3_enabled.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let queue_encryption_key_type_binding = args
            .queue_encryption_key_type
            .get_inner();
        let queue_properties_binding = args.queue_properties.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let routing_binding = args.routing.get_inner();
        let sas_policy_binding = args.sas_policy.get_inner();
        let sftp_enabled_binding = args.sftp_enabled.get_inner();
        let share_properties_binding = args.share_properties.get_inner();
        let shared_access_key_enabled_binding = args
            .shared_access_key_enabled
            .get_inner();
        let static_website_binding = args.static_website.get_inner();
        let table_encryption_key_type_binding = args
            .table_encryption_key_type
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessTier".into(),
                    value: &access_tier_binding,
                },
                register_interface::ObjectField {
                    name: "accountKind".into(),
                    value: &account_kind_binding,
                },
                register_interface::ObjectField {
                    name: "accountReplicationType".into(),
                    value: &account_replication_type_binding,
                },
                register_interface::ObjectField {
                    name: "accountTier".into(),
                    value: &account_tier_binding,
                },
                register_interface::ObjectField {
                    name: "allowNestedItemsToBePublic".into(),
                    value: &allow_nested_items_to_be_public_binding,
                },
                register_interface::ObjectField {
                    name: "allowedCopyScope".into(),
                    value: &allowed_copy_scope_binding,
                },
                register_interface::ObjectField {
                    name: "azureFilesAuthentication".into(),
                    value: &azure_files_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "blobProperties".into(),
                    value: &blob_properties_binding,
                },
                register_interface::ObjectField {
                    name: "crossTenantReplicationEnabled".into(),
                    value: &cross_tenant_replication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "customDomain".into(),
                    value: &custom_domain_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "defaultToOauthAuthentication".into(),
                    value: &default_to_oauth_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "dnsEndpointType".into(),
                    value: &dns_endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "httpsTrafficOnlyEnabled".into(),
                    value: &https_traffic_only_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "immutabilityPolicy".into(),
                    value: &immutability_policy_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureEncryptionEnabled".into(),
                    value: &infrastructure_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "isHnsEnabled".into(),
                    value: &is_hns_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "largeFileShareEnabled".into(),
                    value: &large_file_share_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "localUserEnabled".into(),
                    value: &local_user_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minTlsVersion".into(),
                    value: &min_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkRules".into(),
                    value: &network_rules_binding,
                },
                register_interface::ObjectField {
                    name: "nfsv3Enabled".into(),
                    value: &nfsv3_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "queueEncryptionKeyType".into(),
                    value: &queue_encryption_key_type_binding,
                },
                register_interface::ObjectField {
                    name: "queueProperties".into(),
                    value: &queue_properties_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routing".into(),
                    value: &routing_binding,
                },
                register_interface::ObjectField {
                    name: "sasPolicy".into(),
                    value: &sas_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sftpEnabled".into(),
                    value: &sftp_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "shareProperties".into(),
                    value: &share_properties_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccessKeyEnabled".into(),
                    value: &shared_access_key_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "staticWebsite".into(),
                    value: &static_website_binding,
                },
                register_interface::ObjectField {
                    name: "tableEncryptionKeyType".into(),
                    value: &table_encryption_key_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessTier".into(),
                },
                register_interface::ResultField {
                    name: "accountKind".into(),
                },
                register_interface::ResultField {
                    name: "accountReplicationType".into(),
                },
                register_interface::ResultField {
                    name: "accountTier".into(),
                },
                register_interface::ResultField {
                    name: "allowNestedItemsToBePublic".into(),
                },
                register_interface::ResultField {
                    name: "allowedCopyScope".into(),
                },
                register_interface::ResultField {
                    name: "azureFilesAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "blobProperties".into(),
                },
                register_interface::ResultField {
                    name: "crossTenantReplicationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "customDomain".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultToOauthAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "dnsEndpointType".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "httpsTrafficOnlyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "immutabilityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "isHnsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "largeFileShareEnabled".into(),
                },
                register_interface::ResultField {
                    name: "localUserEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRules".into(),
                },
                register_interface::ResultField {
                    name: "nfsv3Enabled".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryBlobMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryDfsEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryDfsHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryDfsInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryDfsInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryDfsMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryDfsMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryFileEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryFileHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryFileInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryFileInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryFileMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryFileMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryLocation".into(),
                },
                register_interface::ResultField {
                    name: "primaryQueueEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryQueueHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryQueueMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryQueueMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryTableEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryTableHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryTableMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryTableMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryWebEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryWebHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryWebInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryWebInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "primaryWebMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "primaryWebMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "queueEncryptionKeyType".into(),
                },
                register_interface::ResultField {
                    name: "queueProperties".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "routing".into(),
                },
                register_interface::ResultField {
                    name: "sasPolicy".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryBlobMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDfsEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDfsHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDfsInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDfsInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDfsMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryDfsMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryFileEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryFileHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryFileInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryFileInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryFileMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryFileMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryLocation".into(),
                },
                register_interface::ResultField {
                    name: "secondaryQueueEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryQueueHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryQueueMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryQueueMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryTableEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryTableHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryTableMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryTableMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWebEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWebHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWebInternetEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWebInternetHost".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWebMicrosoftEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "secondaryWebMicrosoftHost".into(),
                },
                register_interface::ResultField {
                    name: "sftpEnabled".into(),
                },
                register_interface::ResultField {
                    name: "shareProperties".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessKeyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "staticWebsite".into(),
                },
                register_interface::ResultField {
                    name: "tableEncryptionKeyType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            access_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessTier").unwrap(),
            ),
            account_kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountKind").unwrap(),
            ),
            account_replication_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountReplicationType").unwrap(),
            ),
            account_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountTier").unwrap(),
            ),
            allow_nested_items_to_be_public: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowNestedItemsToBePublic").unwrap(),
            ),
            allowed_copy_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedCopyScope").unwrap(),
            ),
            azure_files_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureFilesAuthentication").unwrap(),
            ),
            blob_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blobProperties").unwrap(),
            ),
            cross_tenant_replication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("crossTenantReplicationEnabled").unwrap(),
            ),
            custom_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomain").unwrap(),
            ),
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKey").unwrap(),
            ),
            default_to_oauth_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultToOauthAuthentication").unwrap(),
            ),
            dns_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsEndpointType").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            https_traffic_only_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsTrafficOnlyEnabled").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            immutability_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("immutabilityPolicy").unwrap(),
            ),
            infrastructure_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureEncryptionEnabled").unwrap(),
            ),
            is_hns_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isHnsEnabled").unwrap(),
            ),
            large_file_share_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("largeFileShareEnabled").unwrap(),
            ),
            local_user_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localUserEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            min_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRules").unwrap(),
            ),
            nfsv3_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nfsv3Enabled").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            primary_blob_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobConnectionString").unwrap(),
            ),
            primary_blob_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobEndpoint").unwrap(),
            ),
            primary_blob_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobHost").unwrap(),
            ),
            primary_blob_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobInternetEndpoint").unwrap(),
            ),
            primary_blob_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobInternetHost").unwrap(),
            ),
            primary_blob_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobMicrosoftEndpoint").unwrap(),
            ),
            primary_blob_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryBlobMicrosoftHost").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            primary_dfs_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDfsEndpoint").unwrap(),
            ),
            primary_dfs_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDfsHost").unwrap(),
            ),
            primary_dfs_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDfsInternetEndpoint").unwrap(),
            ),
            primary_dfs_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDfsInternetHost").unwrap(),
            ),
            primary_dfs_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDfsMicrosoftEndpoint").unwrap(),
            ),
            primary_dfs_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryDfsMicrosoftHost").unwrap(),
            ),
            primary_file_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryFileEndpoint").unwrap(),
            ),
            primary_file_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryFileHost").unwrap(),
            ),
            primary_file_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryFileInternetEndpoint").unwrap(),
            ),
            primary_file_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryFileInternetHost").unwrap(),
            ),
            primary_file_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryFileMicrosoftEndpoint").unwrap(),
            ),
            primary_file_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryFileMicrosoftHost").unwrap(),
            ),
            primary_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryLocation").unwrap(),
            ),
            primary_queue_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryQueueEndpoint").unwrap(),
            ),
            primary_queue_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryQueueHost").unwrap(),
            ),
            primary_queue_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryQueueMicrosoftEndpoint").unwrap(),
            ),
            primary_queue_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryQueueMicrosoftHost").unwrap(),
            ),
            primary_table_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryTableEndpoint").unwrap(),
            ),
            primary_table_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryTableHost").unwrap(),
            ),
            primary_table_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryTableMicrosoftEndpoint").unwrap(),
            ),
            primary_table_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryTableMicrosoftHost").unwrap(),
            ),
            primary_web_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWebEndpoint").unwrap(),
            ),
            primary_web_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWebHost").unwrap(),
            ),
            primary_web_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWebInternetEndpoint").unwrap(),
            ),
            primary_web_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWebInternetHost").unwrap(),
            ),
            primary_web_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWebMicrosoftEndpoint").unwrap(),
            ),
            primary_web_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryWebMicrosoftHost").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            queue_encryption_key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueEncryptionKeyType").unwrap(),
            ),
            queue_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueProperties").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routing").unwrap(),
            ),
            sas_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sasPolicy").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            secondary_blob_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobConnectionString").unwrap(),
            ),
            secondary_blob_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobEndpoint").unwrap(),
            ),
            secondary_blob_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobHost").unwrap(),
            ),
            secondary_blob_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobInternetEndpoint").unwrap(),
            ),
            secondary_blob_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobInternetHost").unwrap(),
            ),
            secondary_blob_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobMicrosoftEndpoint").unwrap(),
            ),
            secondary_blob_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryBlobMicrosoftHost").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            secondary_dfs_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDfsEndpoint").unwrap(),
            ),
            secondary_dfs_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDfsHost").unwrap(),
            ),
            secondary_dfs_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDfsInternetEndpoint").unwrap(),
            ),
            secondary_dfs_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDfsInternetHost").unwrap(),
            ),
            secondary_dfs_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDfsMicrosoftEndpoint").unwrap(),
            ),
            secondary_dfs_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryDfsMicrosoftHost").unwrap(),
            ),
            secondary_file_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryFileEndpoint").unwrap(),
            ),
            secondary_file_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryFileHost").unwrap(),
            ),
            secondary_file_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryFileInternetEndpoint").unwrap(),
            ),
            secondary_file_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryFileInternetHost").unwrap(),
            ),
            secondary_file_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryFileMicrosoftEndpoint").unwrap(),
            ),
            secondary_file_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryFileMicrosoftHost").unwrap(),
            ),
            secondary_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryLocation").unwrap(),
            ),
            secondary_queue_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryQueueEndpoint").unwrap(),
            ),
            secondary_queue_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryQueueHost").unwrap(),
            ),
            secondary_queue_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryQueueMicrosoftEndpoint").unwrap(),
            ),
            secondary_queue_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryQueueMicrosoftHost").unwrap(),
            ),
            secondary_table_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryTableEndpoint").unwrap(),
            ),
            secondary_table_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryTableHost").unwrap(),
            ),
            secondary_table_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryTableMicrosoftEndpoint").unwrap(),
            ),
            secondary_table_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryTableMicrosoftHost").unwrap(),
            ),
            secondary_web_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWebEndpoint").unwrap(),
            ),
            secondary_web_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWebHost").unwrap(),
            ),
            secondary_web_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWebInternetEndpoint").unwrap(),
            ),
            secondary_web_internet_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWebInternetHost").unwrap(),
            ),
            secondary_web_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWebMicrosoftEndpoint").unwrap(),
            ),
            secondary_web_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryWebMicrosoftHost").unwrap(),
            ),
            sftp_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sftpEnabled").unwrap(),
            ),
            share_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareProperties").unwrap(),
            ),
            shared_access_key_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessKeyEnabled").unwrap(),
            ),
            static_website: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticWebsite").unwrap(),
            ),
            table_encryption_key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableEncryptionKeyType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}