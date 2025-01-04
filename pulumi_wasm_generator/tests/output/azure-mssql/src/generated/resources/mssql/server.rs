/// Manages a Microsoft SQL Azure Database Server.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: database-rg
///       location: West Europe
///   exampleServer:
///     type: azure:mssql:Server
///     name: example
///     properties:
///       name: mssqlserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '12.0'
///       administratorLogin: missadministrator
///       administratorLoginPassword: thisIsKat11
///       minimumTlsVersion: '1.2'
///       azureadAdministrator:
///         loginUsername: AzureAD Admin
///         objectId: 00000000-0000-0000-0000-000000000000
///       tags:
///         environment: production
/// ```
///
///
/// ### Transparent Data Encryption(TDE) With A Customer Managed Key(CMK) During Create
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-admin
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleServer:
///     type: azure:mssql:Server
///     name: example
///     properties:
///       name: example-resource
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '12.0'
///       administratorLogin: Example-Administrator
///       administratorLoginPassword: Example_Password!
///       minimumTlsVersion: '1.2'
///       azureadAdministrator:
///         loginUsername: ${exampleUserAssignedIdentity.name}
///         objectId: ${exampleUserAssignedIdentity.principalId}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       primaryUserAssignedIdentityId: ${exampleUserAssignedIdentity.id}
///       transparentDataEncryptionKeyVaultKeyId: ${exampleKey.id}
///   # Create a key vault with access policies which allow for the current user to get, list, create, delete, update, recover, purge and getRotationPolicy for the key vault key and also add a key vault access policy for the Microsoft Sql Server instance User Managed Identity to get, wrap, and unwrap key(s)
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: mssqltdeexample
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       enabledForDiskEncryption: true
///       tenantId: ${exampleUserAssignedIdentity.tenantId}
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: true
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Get
///             - List
///             - Create
///             - Delete
///             - Update
///             - Recover
///             - Purge
///             - GetRotationPolicy
///         - tenantId: ${exampleUserAssignedIdentity.tenantId}
///           objectId: ${exampleUserAssignedIdentity.principalId}
///           keyPermissions:
///             - Get
///             - WrapKey
///             - UnwrapKey
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: example-key
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - unwrapKey
///         - wrapKey
///     options:
///       dependsOn:
///         - ${exampleKeyVault}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// SQL Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/server:Server example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/servers/myserver
/// ```
///
pub mod server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerArgs {
        /// The administrator login name for the new server. Required unless `azuread_authentication_only` in the `azuread_administrator` block is `true`. When omitted, Azure will generate a default username which cannot be subsequently changed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub administrator_login: pulumi_wasm_rust::Output<Option<String>>,
        /// The password associated with the `administrator_login` user. Needs to comply with Azure's [Password Policy](https://msdn.microsoft.com/library/ms161959.aspx). Required unless `azuread_authentication_only` in the `azuread_administrator` block is `true`.
        #[builder(into, default)]
        pub administrator_login_password: pulumi_wasm_rust::Output<Option<String>>,
        /// An `azuread_administrator` block as defined below.
        #[builder(into, default)]
        pub azuread_administrator: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::ServerAzureadAdministrator>,
        >,
        /// The connection policy the server will use. Possible values are `Default`, `Proxy`, and `Redirect`. Defaults to `Default`.
        #[builder(into, default)]
        pub connection_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::ServerIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The Minimum TLS Version for all SQL Database and SQL Data Warehouse databases associated with the server. Valid values are: `1.0`, `1.1` , `1.2` and `Disabled`. Defaults to `1.2`.
        ///
        /// > **NOTE:** The `minimum_tls_version` is set to `Disabled` means all TLS versions are allowed. After you enforce a version of `minimum_tls_version`, it's not possible to revert to `Disabled`.
        ///
        /// > **NOTE:** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Microsoft SQL Server. This needs to be globally unique within Azure. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether outbound network traffic is restricted for this server. Defaults to `false`.
        #[builder(into, default)]
        pub outbound_network_restriction_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the primary user managed identity id. Required if `type` within the `identity` block is set to either `SystemAssigned, UserAssigned` or `UserAssigned` and should be set at same time as setting `identity_ids`.
        #[builder(into, default)]
        pub primary_user_assigned_identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed for this server. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Microsoft SQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The fully versioned `Key Vault` `Key` URL (e.g. `'https://<YourVaultName>.vault.azure.net/keys/<YourKeyName>/<YourKeyVersion>`) to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        ///
        /// > **NOTE:** To successfully deploy a `Microsoft SQL Server` in CMK/BYOK TDE the `Key Vault` must have `Soft-delete` and `purge protection` enabled to protect from data loss due to accidental key and/or key vault deletion. The `Key Vault` and the `Microsoft SQL Server` `User Managed Identity Instance` must belong to the same `Azure Active Directory` `tenant`.
        ///
        /// > **NOTE:**  Cross-tenant `Key Vault` and `Microsoft SQL Server` interactions are not supported. Please see the [product documentation](https://learn.microsoft.com/azure/azure-sql/database/transparent-data-encryption-byok-overview?view=azuresql#requirements-for-configuring-customer-managed-tde) for more information.
        ///
        /// > **NOTE:** When using a firewall with a `Key Vault`, you must enable the option `Allow trusted Microsoft services to bypass the firewall`.
        #[builder(into, default)]
        pub transparent_data_encryption_key_vault_key_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The version for the new server. Valid values are: 2.0 (for v11 server) and 12.0 (for v12 server). Changing this forces a new resource to be created.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServerResult {
        /// The administrator login name for the new server. Required unless `azuread_authentication_only` in the `azuread_administrator` block is `true`. When omitted, Azure will generate a default username which cannot be subsequently changed. Changing this forces a new resource to be created.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The password associated with the `administrator_login` user. Needs to comply with Azure's [Password Policy](https://msdn.microsoft.com/library/ms161959.aspx). Required unless `azuread_authentication_only` in the `azuread_administrator` block is `true`.
        pub administrator_login_password: pulumi_wasm_rust::Output<Option<String>>,
        /// An `azuread_administrator` block as defined below.
        pub azuread_administrator: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::ServerAzureadAdministrator>,
        >,
        /// The connection policy the server will use. Possible values are `Default`, `Proxy`, and `Redirect`. Defaults to `Default`.
        pub connection_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The fully qualified domain name of the Azure SQL Server (e.g. myServerName.database.windows.net)
        pub fully_qualified_domain_name: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::ServerIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Minimum TLS Version for all SQL Database and SQL Data Warehouse databases associated with the server. Valid values are: `1.0`, `1.1` , `1.2` and `Disabled`. Defaults to `1.2`.
        ///
        /// > **NOTE:** The `minimum_tls_version` is set to `Disabled` means all TLS versions are allowed. After you enforce a version of `minimum_tls_version`, it's not possible to revert to `Disabled`.
        ///
        /// > **NOTE:** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Microsoft SQL Server. This needs to be globally unique within Azure. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether outbound network traffic is restricted for this server. Defaults to `false`.
        pub outbound_network_restriction_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the primary user managed identity id. Required if `type` within the `identity` block is set to either `SystemAssigned, UserAssigned` or `UserAssigned` and should be set at same time as setting `identity_ids`.
        pub primary_user_assigned_identity_id: pulumi_wasm_rust::Output<String>,
        /// Whether public network access is allowed for this server. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Microsoft SQL Server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of dropped restorable database IDs on the server.
        pub restorable_dropped_database_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The fully versioned `Key Vault` `Key` URL (e.g. `'https://<YourVaultName>.vault.azure.net/keys/<YourKeyName>/<YourKeyVersion>`) to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        ///
        /// > **NOTE:** To successfully deploy a `Microsoft SQL Server` in CMK/BYOK TDE the `Key Vault` must have `Soft-delete` and `purge protection` enabled to protect from data loss due to accidental key and/or key vault deletion. The `Key Vault` and the `Microsoft SQL Server` `User Managed Identity Instance` must belong to the same `Azure Active Directory` `tenant`.
        ///
        /// > **NOTE:**  Cross-tenant `Key Vault` and `Microsoft SQL Server` interactions are not supported. Please see the [product documentation](https://learn.microsoft.com/azure/azure-sql/database/transparent-data-encryption-byok-overview?view=azuresql#requirements-for-configuring-customer-managed-tde) for more information.
        ///
        /// > **NOTE:** When using a firewall with a `Key Vault`, you must enable the option `Allow trusted Microsoft services to bypass the firewall`.
        pub transparent_data_encryption_key_vault_key_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The version for the new server. Valid values are: 2.0 (for v11 server) and 12.0 (for v12 server). Changing this forces a new resource to be created.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServerArgs) -> ServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_login_binding = args.administrator_login.get_inner();
        let administrator_login_password_binding = args
            .administrator_login_password
            .get_inner();
        let azuread_administrator_binding = args.azuread_administrator.get_inner();
        let connection_policy_binding = args.connection_policy.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let minimum_tls_version_binding = args.minimum_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let outbound_network_restriction_enabled_binding = args
            .outbound_network_restriction_enabled
            .get_inner();
        let primary_user_assigned_identity_id_binding = args
            .primary_user_assigned_identity_id
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let transparent_data_encryption_key_vault_key_id_binding = args
            .transparent_data_encryption_key_vault_key_id
            .get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/server:Server".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorLogin".into(),
                    value: &administrator_login_binding,
                },
                register_interface::ObjectField {
                    name: "administratorLoginPassword".into(),
                    value: &administrator_login_password_binding,
                },
                register_interface::ObjectField {
                    name: "azureadAdministrator".into(),
                    value: &azuread_administrator_binding,
                },
                register_interface::ObjectField {
                    name: "connectionPolicy".into(),
                    value: &connection_policy_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outboundNetworkRestrictionEnabled".into(),
                    value: &outbound_network_restriction_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "primaryUserAssignedIdentityId".into(),
                    value: &primary_user_assigned_identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transparentDataEncryptionKeyVaultKeyId".into(),
                    value: &transparent_data_encryption_key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "administratorLoginPassword".into(),
                },
                register_interface::ResultField {
                    name: "azureadAdministrator".into(),
                },
                register_interface::ResultField {
                    name: "connectionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "fullyQualifiedDomainName".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundNetworkRestrictionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "primaryUserAssignedIdentityId".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            administrator_login_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLoginPassword").unwrap(),
            ),
            azuread_administrator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureadAdministrator").unwrap(),
            ),
            connection_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionPolicy").unwrap(),
            ),
            fully_qualified_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fullyQualifiedDomainName").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_network_restriction_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundNetworkRestrictionEnabled").unwrap(),
            ),
            primary_user_assigned_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryUserAssignedIdentityId").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
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
