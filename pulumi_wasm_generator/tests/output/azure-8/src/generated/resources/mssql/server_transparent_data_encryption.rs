/// Manages the transparent data encryption configuration for a MSSQL Server
///
/// !> **IMPORTANT:** This resource can be used to configure Transparent Data Encryption for MS SQL instances with Customer Managed Keys. For MS SQL instances that are System Managed, it should only be used with pre-existing MS SQL Instances that are over 3 years old. For new System Managed MS SQL Instances that will be created through the use of the `azure.mssql.Server` resource, please enable Transparent Data Encryption through `azure.mssql.Server` resource itself by configuring an identity block. By default, all new MS SQL Instances are deployed with System Managed Transparent Data Encryption enabled.
///
/// > **NOTE:** Once transparent data encryption is enabled on a MS SQL instance, it is not possible to remove TDE. You will be able to switch between 'ServiceManaged' and 'CustomerManaged' keys, but will not be able to remove encryption. For safety when this resource is deleted, the TDE mode will automatically be set to 'ServiceManaged'. See `key_vault_uri` for more information on how to specify the key types. As SQL Server only supports a single configuration for encryption settings, this resource will replace the current encryption settings on the server.
///
/// > **Note:** See [documentation](https://docs.microsoft.com/azure/azure-sql/database/transparent-data-encryption-byok-overview) for important information on how handle lifecycle management of the keys to prevent data lockout.
///
/// ## Example Usage
///
/// ### With Service Managed Key
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: EastUs
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
///   exampleServerTransparentDataEncryption:
///     type: azure:mssql:ServerTransparentDataEncryption
///     name: example
///     properties:
///       serverId: ${exampleServer.id}
/// ```
///
///
/// ### With Customer Managed Key
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: EastUs
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
///       identity:
///         type: SystemAssigned
///   # Create a key vault with policies for the deployer to create a key & SQL Server to wrap/unwrap/get key
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       enabledForDiskEncryption: true
///       tenantId: ${current.tenantId}
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: false
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
///         - tenantId: ${exampleServer.identity.tenantId}
///           objectId: ${exampleServer.identity.principalId}
///           keyPermissions:
///             - Get
///             - WrapKey
///             - UnwrapKey
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: byok
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - unwrapKey
///         - wrapKey
///     options:
///       dependsOn:
///         - ${exampleKeyVault}
///   exampleServerTransparentDataEncryption:
///     type: azure:mssql:ServerTransparentDataEncryption
///     name: example
///     properties:
///       serverId: ${exampleServer.id}
///       keyVaultKeyId: ${exampleKey.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// SQL Server Transparent Data Encryption can be imported using the resource id, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/serverTransparentDataEncryption:ServerTransparentDataEncryption example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Sql/servers/server1/encryptionProtector/current
/// ```
///
pub mod server_transparent_data_encryption {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerTransparentDataEncryptionArgs {
        /// When enabled, the server will continuously check the key vault for any new versions of the key being used as the TDE protector. If a new version of the key is detected, the TDE protector on the server will be automatically rotated to the latest key version within 60 minutes.
        #[builder(into, default)]
        pub auto_rotation_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// To use customer managed keys from Azure Key Vault, provide the AKV Key ID. To use service managed keys, omit this field.
        ///
        /// > **NOTE:** In order to use customer managed keys, the identity of the MSSQL server must have the following permissions on the key vault: 'get', 'wrapKey' and 'unwrapKey'
        ///
        /// > **NOTE:** If `server_id` denotes a secondary server deployed for disaster recovery purposes, then the `key_vault_key_id` should be the same key used for the primary server's transparent data encryption. Both primary and secondary servers should be encrypted with same key material.
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub managed_hsm_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the MS SQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerTransparentDataEncryptionResult {
        /// When enabled, the server will continuously check the key vault for any new versions of the key being used as the TDE protector. If a new version of the key is detected, the TDE protector on the server will be automatically rotated to the latest key version within 60 minutes.
        pub auto_rotation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// To use customer managed keys from Azure Key Vault, provide the AKV Key ID. To use service managed keys, omit this field.
        ///
        /// > **NOTE:** In order to use customer managed keys, the identity of the MSSQL server must have the following permissions on the key vault: 'get', 'wrapKey' and 'unwrapKey'
        ///
        /// > **NOTE:** If `server_id` denotes a secondary server deployed for disaster recovery purposes, then the `key_vault_key_id` should be the same key used for the primary server's transparent data encryption. Both primary and secondary servers should be encrypted with same key material.
        pub key_vault_key_id: pulumi_wasm_rust::Output<Option<String>>,
        pub managed_hsm_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the MS SQL Server. Changing this forces a new resource to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerTransparentDataEncryptionArgs,
    ) -> ServerTransparentDataEncryptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_rotation_enabled_binding = args
            .auto_rotation_enabled
            .get_output(context)
            .get_inner();
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let managed_hsm_key_id_binding = args
            .managed_hsm_key_id
            .get_output(context)
            .get_inner();
        let server_id_binding = args.server_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/serverTransparentDataEncryption:ServerTransparentDataEncryption"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoRotationEnabled".into(),
                    value: &auto_rotation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "managedHsmKeyId".into(),
                    value: &managed_hsm_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerTransparentDataEncryptionResult {
            auto_rotation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoRotationEnabled"),
            ),
            key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            managed_hsm_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedHsmKeyId"),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverId"),
            ),
        }
    }
}
