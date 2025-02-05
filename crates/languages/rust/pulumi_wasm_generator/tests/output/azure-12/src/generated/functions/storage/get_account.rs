pub mod get_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// The minimum supported TLS version for this storage account.
        #[builder(into, default)]
        pub min_tls_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Storage Account
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Storage Account is located in.
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// The access tier for `BlobStorage` accounts.
        pub access_tier: pulumi_wasm_rust::Output<String>,
        /// The Kind of account.
        pub account_kind: pulumi_wasm_rust::Output<String>,
        /// The type of replication used for this storage account.
        pub account_replication_type: pulumi_wasm_rust::Output<String>,
        /// The Tier of this storage account.
        pub account_tier: pulumi_wasm_rust::Output<String>,
        /// Can nested items in the storage account opt into allowing public access?
        pub allow_nested_items_to_be_public: pulumi_wasm_rust::Output<bool>,
        /// A `azure_files_authentication` block as documented below.
        pub azure_files_authentications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetAccountAzureFilesAuthentication>,
        >,
        /// supports the following:
        pub custom_domains: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetAccountCustomDomain>,
        >,
        /// Which DNS endpoint type is used - either `Standard` or `AzureDnsZone`.
        pub dns_endpoint_type: pulumi_wasm_rust::Output<String>,
        /// Is traffic only allowed via HTTPS? See [here](https://docs.microsoft.com/azure/storage/storage-require-secure-transfer/) for more information.
        pub https_traffic_only_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as documented below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetAccountIdentity>,
        >,
        /// Is infrastructure encryption enabled? See [here](https://docs.microsoft.com/azure/storage/common/infrastructure-encryption-enable/)
        /// for more information.
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        /// Is Hierarchical Namespace enabled?
        pub is_hns_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure location where the Storage Account exists
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum supported TLS version for this storage account.
        pub min_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The Custom Domain Name used for the Storage Account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is NFSv3 protocol enabled?
        pub nfsv3_enabled: pulumi_wasm_rust::Output<bool>,
        /// The primary access key for the Storage Account.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The connection string associated with the primary blob location
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
        /// The connection string associated with the primary location
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
        /// The primary location of the Storage Account.
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
        /// The encryption key type of the queue.
        pub queue_encryption_key_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The secondary access key for the Storage Account.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The connection string associated with the secondary blob location
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
        /// The connection string associated with the secondary location
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
        /// The secondary location of the Storage Account.
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
        /// The encryption key type of the table.
        pub table_encryption_key_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
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
        let min_tls_version_binding = args
            .min_tls_version
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getAccount:getAccount".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "minTlsVersion".into(),
                    value: &min_tls_version_binding,
                },
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
            access_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessTier"),
            ),
            account_kind: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountKind"),
            ),
            account_replication_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountReplicationType"),
            ),
            account_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountTier"),
            ),
            allow_nested_items_to_be_public: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowNestedItemsToBePublic"),
            ),
            azure_files_authentications: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("azureFilesAuthentications"),
            ),
            custom_domains: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDomains"),
            ),
            dns_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsEndpointType"),
            ),
            https_traffic_only_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpsTrafficOnlyEnabled"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            infrastructure_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("infrastructureEncryptionEnabled"),
            ),
            is_hns_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isHnsEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            min_tls_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minTlsVersion"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            nfsv3_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nfsv3Enabled"),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            primary_blob_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobConnectionString"),
            ),
            primary_blob_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobEndpoint"),
            ),
            primary_blob_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobHost"),
            ),
            primary_blob_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobInternetEndpoint"),
            ),
            primary_blob_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobInternetHost"),
            ),
            primary_blob_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobMicrosoftEndpoint"),
            ),
            primary_blob_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryBlobMicrosoftHost"),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            primary_dfs_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryDfsEndpoint"),
            ),
            primary_dfs_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryDfsHost"),
            ),
            primary_dfs_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryDfsInternetEndpoint"),
            ),
            primary_dfs_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryDfsInternetHost"),
            ),
            primary_dfs_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryDfsMicrosoftEndpoint"),
            ),
            primary_dfs_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryDfsMicrosoftHost"),
            ),
            primary_file_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryFileEndpoint"),
            ),
            primary_file_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryFileHost"),
            ),
            primary_file_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryFileInternetEndpoint"),
            ),
            primary_file_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryFileInternetHost"),
            ),
            primary_file_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryFileMicrosoftEndpoint"),
            ),
            primary_file_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryFileMicrosoftHost"),
            ),
            primary_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryLocation"),
            ),
            primary_queue_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryQueueEndpoint"),
            ),
            primary_queue_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryQueueHost"),
            ),
            primary_queue_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryQueueMicrosoftEndpoint"),
            ),
            primary_queue_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryQueueMicrosoftHost"),
            ),
            primary_table_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryTableEndpoint"),
            ),
            primary_table_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryTableHost"),
            ),
            primary_table_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryTableMicrosoftEndpoint"),
            ),
            primary_table_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryTableMicrosoftHost"),
            ),
            primary_web_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWebEndpoint"),
            ),
            primary_web_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWebHost"),
            ),
            primary_web_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWebInternetEndpoint"),
            ),
            primary_web_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWebInternetHost"),
            ),
            primary_web_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWebMicrosoftEndpoint"),
            ),
            primary_web_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryWebMicrosoftHost"),
            ),
            queue_encryption_key_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueEncryptionKeyType"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            secondary_blob_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobConnectionString"),
            ),
            secondary_blob_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobEndpoint"),
            ),
            secondary_blob_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobHost"),
            ),
            secondary_blob_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobInternetEndpoint"),
            ),
            secondary_blob_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobInternetHost"),
            ),
            secondary_blob_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobMicrosoftEndpoint"),
            ),
            secondary_blob_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryBlobMicrosoftHost"),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            secondary_dfs_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryDfsEndpoint"),
            ),
            secondary_dfs_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryDfsHost"),
            ),
            secondary_dfs_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryDfsInternetEndpoint"),
            ),
            secondary_dfs_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryDfsInternetHost"),
            ),
            secondary_dfs_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryDfsMicrosoftEndpoint"),
            ),
            secondary_dfs_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryDfsMicrosoftHost"),
            ),
            secondary_file_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryFileEndpoint"),
            ),
            secondary_file_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryFileHost"),
            ),
            secondary_file_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryFileInternetEndpoint"),
            ),
            secondary_file_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryFileInternetHost"),
            ),
            secondary_file_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryFileMicrosoftEndpoint"),
            ),
            secondary_file_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryFileMicrosoftHost"),
            ),
            secondary_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryLocation"),
            ),
            secondary_queue_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryQueueEndpoint"),
            ),
            secondary_queue_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryQueueHost"),
            ),
            secondary_queue_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryQueueMicrosoftEndpoint"),
            ),
            secondary_queue_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryQueueMicrosoftHost"),
            ),
            secondary_table_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryTableEndpoint"),
            ),
            secondary_table_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryTableHost"),
            ),
            secondary_table_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryTableMicrosoftEndpoint"),
            ),
            secondary_table_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryTableMicrosoftHost"),
            ),
            secondary_web_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWebEndpoint"),
            ),
            secondary_web_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWebHost"),
            ),
            secondary_web_internet_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWebInternetEndpoint"),
            ),
            secondary_web_internet_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWebInternetHost"),
            ),
            secondary_web_microsoft_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWebMicrosoftEndpoint"),
            ),
            secondary_web_microsoft_host: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryWebMicrosoftHost"),
            ),
            table_encryption_key_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableEncryptionKeyType"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
