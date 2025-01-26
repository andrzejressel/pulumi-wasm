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
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
                    name: "azureFilesAuthentications".into(),
                },
                register_interface::ResultField {
                    name: "customDomains".into(),
                },
                register_interface::ResultField {
                    name: "dnsEndpointType".into(),
                },
                register_interface::ResultField {
                    name: "httpsTrafficOnlyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "isHnsEnabled".into(),
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
                    name: "queueEncryptionKeyType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
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
                    name: "tableEncryptionKeyType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountResult {
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
            azure_files_authentications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureFilesAuthentications").unwrap(),
            ),
            custom_domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomains").unwrap(),
            ),
            dns_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsEndpointType").unwrap(),
            ),
            https_traffic_only_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsTrafficOnlyEnabled").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            infrastructure_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureEncryptionEnabled").unwrap(),
            ),
            is_hns_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isHnsEnabled").unwrap(),
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
            queue_encryption_key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueEncryptionKeyType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
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
            table_encryption_key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableEncryptionKeyType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
