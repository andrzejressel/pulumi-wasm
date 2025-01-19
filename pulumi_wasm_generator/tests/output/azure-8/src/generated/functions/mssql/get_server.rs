pub mod get_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerArgs {
        /// The name of this Microsoft SQL Server.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Microsoft SQL Server exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerResult {
        /// The server's administrator login name.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The fully qualified domain name of the Azure SQL Server.
        pub fully_qualified_domain_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mssql::GetServerIdentity>,
        >,
        /// The Azure Region where the Microsoft SQL Server exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of dropped restorable database IDs on the server.
        pub restorable_dropped_database_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags assigned to this Microsoft SQL Server.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Key Vault key URI to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        pub transparent_data_encryption_key_vault_key_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// This servers MS SQL version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServerArgs) -> GetServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mssql/getServer:getServer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "fullyQualifiedDomainName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "restorableDroppedDatabaseIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transparentDataEncryptionKeyVaultKeyId".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            fully_qualified_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullyQualifiedDomainName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            restorable_dropped_database_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorableDroppedDatabaseIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transparent_data_encryption_key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transparentDataEncryptionKeyVaultKeyId").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
