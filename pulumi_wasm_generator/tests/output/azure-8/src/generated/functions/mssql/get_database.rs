pub mod get_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseArgs {
        /// The name of the MS SQL Database.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the MS SQL Server on which to read the database.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseResult {
        /// The collation of the database.
        pub collation: pulumi_wasm_rust::Output<String>,
        /// The id of the elastic pool containing this database.
        pub elastic_pool_id: pulumi_wasm_rust::Output<String>,
        /// The type of enclave being used by the database.
        pub enclave_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mssql::GetDatabaseIdentity>,
        >,
        /// The license type to apply for this database.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// The max size of the database in gigabytes.
        pub max_size_gb: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of readonly secondary replicas associated with the database to which readonly application intent connections may be routed.
        pub read_replica_count: pulumi_wasm_rust::Output<i32>,
        /// If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica.
        pub read_scale: pulumi_wasm_rust::Output<bool>,
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The name of the SKU of the database.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The storage account type used to store backups for this database.
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether or not Transparent Data Encryption is enabled.
        pub transparent_data_encryption_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether or not TDE automatically rotates the encryption Key to latest version.
        pub transparent_data_encryption_key_automatic_rotation_enabled: pulumi_wasm_rust::Output<
            bool,
        >,
        /// The Key Vault key URI to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        pub transparent_data_encryption_key_vault_key_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// Whether or not this database is zone redundant, which means the replicas of this database will be spread across multiple availability zones.
        pub zone_redundant: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDatabaseArgs,
    ) -> GetDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let server_id_binding = args.server_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mssql/getDatabase:getDatabase".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "collation".into(),
                },
                register_interface::ResultField {
                    name: "elasticPoolId".into(),
                },
                register_interface::ResultField {
                    name: "enclaveType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "maxSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "readReplicaCount".into(),
                },
                register_interface::ResultField {
                    name: "readScale".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transparentDataEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "transparentDataEncryptionKeyAutomaticRotationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "transparentDataEncryptionKeyVaultKeyId".into(),
                },
                register_interface::ResultField {
                    name: "zoneRedundant".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatabaseResult {
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
            ),
            elastic_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticPoolId").unwrap(),
            ),
            enclave_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enclaveType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            max_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSizeGb").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            read_replica_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readReplicaCount").unwrap(),
            ),
            read_scale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readScale").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_account_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transparent_data_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transparentDataEncryptionEnabled").unwrap(),
            ),
            transparent_data_encryption_key_automatic_rotation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap
                    .remove("transparentDataEncryptionKeyAutomaticRotationEnabled")
                    .unwrap(),
            ),
            transparent_data_encryption_key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transparentDataEncryptionKeyVaultKeyId").unwrap(),
            ),
            zone_redundant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneRedundant").unwrap(),
            ),
        }
    }
}
