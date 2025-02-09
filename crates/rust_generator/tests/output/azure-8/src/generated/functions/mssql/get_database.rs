#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseArgs {
        /// The name of the MS SQL Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the MS SQL Server on which to read the database.
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseResult {
        /// The collation of the database.
        pub collation: pulumi_gestalt_rust::Output<String>,
        /// The id of the elastic pool containing this database.
        pub elastic_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The type of enclave being used by the database.
        pub enclave_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mssql::GetDatabaseIdentity>,
        >,
        /// The license type to apply for this database.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// The max size of the database in gigabytes.
        pub max_size_gb: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of readonly secondary replicas associated with the database to which readonly application intent connections may be routed.
        pub read_replica_count: pulumi_gestalt_rust::Output<i32>,
        /// If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica.
        pub read_scale: pulumi_gestalt_rust::Output<bool>,
        pub server_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the SKU of the database.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The storage account type used to store backups for this database.
        pub storage_account_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether or not Transparent Data Encryption is enabled.
        pub transparent_data_encryption_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not TDE automatically rotates the encryption Key to latest version.
        pub transparent_data_encryption_key_automatic_rotation_enabled: pulumi_gestalt_rust::Output<
            bool,
        >,
        /// The Key Vault key URI to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        pub transparent_data_encryption_key_vault_key_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Whether or not this database is zone redundant, which means the replicas of this database will be spread across multiple availability zones.
        pub zone_redundant: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDatabaseArgs,
    ) -> GetDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let server_id_binding_1 = args.server_id.get_output(context);
        let server_id_binding = server_id_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatabaseResult {
            collation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("collation"),
            ),
            elastic_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("elasticPoolId"),
            ),
            enclave_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enclaveType"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            license_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseType"),
            ),
            max_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxSizeGb"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            read_replica_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readReplicaCount"),
            ),
            read_scale: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readScale"),
            ),
            server_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverId"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            storage_account_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            transparent_data_encryption_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transparentDataEncryptionEnabled"),
            ),
            transparent_data_encryption_key_automatic_rotation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transparentDataEncryptionKeyAutomaticRotationEnabled"),
            ),
            transparent_data_encryption_key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transparentDataEncryptionKeyVaultKeyId"),
            ),
            zone_redundant: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneRedundant"),
            ),
        }
    }
}
