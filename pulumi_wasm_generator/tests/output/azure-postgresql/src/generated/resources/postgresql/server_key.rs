/// Manages a Customer Managed Key for a PostgreSQL Server.
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
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       purgeProtectionEnabled: true
///   server:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${exampleServer.identity.principalId}
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///       secretPermissions:
///         - Get
///   client:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Get
///         - Create
///         - Delete
///         - List
///         - Restore
///         - Recover
///         - UnwrapKey
///         - WrapKey
///         - Purge
///         - Encrypt
///         - Decrypt
///         - Sign
///         - Verify
///         - GetRotationPolicy
///       secretPermissions:
///         - Get
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: tfex-key
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///     options:
///       dependsOn:
///         - ${client}
///         - ${server}
///   exampleServer:
///     type: azure:postgresql:Server
///     name: example
///     properties:
///       name: example-postgre-server
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       administratorLogin: psqladmin
///       administratorLoginPassword: H@Sh1CoR3!
///       skuName: GP_Gen5_2
///       version: '11'
///       storageMb: 51200
///       sslEnforcementEnabled: true
///       identity:
///         type: SystemAssigned
///   exampleServerKey:
///     type: azure:postgresql:ServerKey
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
/// A PostgreSQL Server Key can be imported using the `resource id` of the PostgreSQL Server Key, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/serverKey:ServerKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforPostgreSQL/servers/server1/keys/keyvaultname_key-name_keyversion
/// ```
///
pub mod server_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerKeyArgs {
        /// The URL to a Key Vault Key.
        #[builder(into)]
        pub key_vault_key_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the PostgreSQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServerKeyResult {
        /// The URL to a Key Vault Key.
        pub key_vault_key_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the PostgreSQL Server. Changing this forces a new resource to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServerKeyArgs) -> ServerKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_vault_key_id_binding = args.key_vault_key_id.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/serverKey:ServerKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "keyVaultKeyId".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerKeyResult {
            key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultKeyId").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
        }
    }
}